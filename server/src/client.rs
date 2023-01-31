#![allow(dead_code)]
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex, MutexGuard},
};

use parser::{
    DeleteResponse, Entry, Request, Response, RetrieveRequest, StoreRequest, StoreResponse,
};

use crate::data_file::Data;

pub struct Client {
    socket: TcpStream,
    store: Arc<Mutex<HashMap<String, Entry>>>,
    store_file: Arc<Mutex<Data>>,
}

impl Client {
    pub fn new(
        socket: TcpStream,
        store: Arc<Mutex<HashMap<String, Entry>>>,
        store_file: Arc<Mutex<Data>>,
    ) -> Self {
        Self {
            socket,
            store,
            store_file,
        }
    }

    pub fn read<'a>(&mut self, buf: &'a mut [u8]) -> Option<&'a str> {
        let n = self.socket.read(buf).unwrap();
        if n == 0 {
            return None;
        }

        let decoded_str = std::str::from_utf8(&buf[..n]).unwrap();
        return Some(decoded_str);
    }

    fn get_store(&self) -> MutexGuard<HashMap<String, Entry>> {
        self.store.lock().unwrap()
    }
    pub fn set(&mut self, data: Entry) -> StoreResponse {
        let key = data.key.clone();
        self.get_store().insert(key, data.clone());
        self.store_file.lock().unwrap().write_data(&data);
        StoreResponse::Stored
    }
    pub fn update_value<F>(&mut self, key: &String, f: F) -> StoreResponse
    where
        F: FnOnce(&mut Entry),
    {
        if let Some(e) = self.get_store().get_mut(key) {
            f(e);
            self.store_file.lock().unwrap().write_data(&e);
            return StoreResponse::Stored;
        };
        StoreResponse::NotStored
    }

    pub fn add(&mut self, data: Entry) -> StoreResponse {
        if !self.get_store().contains_key(&data.key) {
            self.set(data.clone());
            self.store_file.lock().unwrap().write_data(&data);
            return StoreResponse::Stored;
        }
        StoreResponse::NotStored
    }

    pub fn replace(&mut self, data: Entry) -> StoreResponse {
        return self.update_value(&data.key, |e: &mut Entry| {
            e.replace(&data);
        });
    }

    pub fn append(&mut self, data: Entry) -> StoreResponse {
        self.update_value(&data.key, |e: &mut Entry| {
            e.append(&data);
        })
    }
    pub fn prepend(&mut self, data: Entry) -> StoreResponse {
        self.update_value(&data.key, |e: &mut Entry| {
            e.prepend(&data);
        })
    }

    pub fn get(&self, key: &String) -> Option<Entry> {
        let lock = self.get_store();
        let value = lock.get(key);
        if let Some(entry) = value {
            return Some(entry.clone());
        }
        return None;
    }

    pub fn handle_connection(&mut self) {
        loop {
            let mut buf = [0u8; 512];
            if let Some(input) = self.read(&mut buf) {
                let request = Request::from_str(input);
                println!("request: {:?}", request);
                let response = self.handle_request(request);
                self.socket.write(response.to_string().as_bytes()).unwrap();
            } else {
                return;
            }
        }
    }
    fn handle_request(&mut self, request: Request) -> Response {
        match request {
            Request::Store(req) => Response::Store(self.handle_store(req)),
            Request::Retreive(req) => self.handle_retrieve(req),
            Request::FlushAll => self.handle_flush_all(),
            Request::Delete(key) => self.handle_delete(key),
        }
    }

    fn handle_store(&mut self, req: StoreRequest) -> StoreResponse {
        let store_response = match req {
            StoreRequest::Set(e) => self.set(e),
            StoreRequest::Add(e) => self.add(e),
            StoreRequest::Replace(e) => self.replace(e),
            StoreRequest::Append(e) => self.append(e),
            StoreRequest::Prepend(e) => self.prepend(e),
        };
        return store_response;
    }
    fn handle_retrieve(&self, req: RetrieveRequest) -> Response {
        match req {
            RetrieveRequest::Get(key) => {
                if let Some(value) = self.get(&key) {
                    Response::Retrieve(value)
                } else {
                    Response::End
                }
            }
        }
    }
    fn handle_flush_all(&mut self) -> Response {
        self.get_store().clear();
        return Response::Ok;
    }

    fn handle_delete(&mut self, key: String) -> Response {
        let mut store = self.get_store();
        if let Some(_) = store.remove(&key) {
            return Response::Delete(DeleteResponse::Deleted);
        }
        return Response::Delete(DeleteResponse::NotFound);
    }
}
