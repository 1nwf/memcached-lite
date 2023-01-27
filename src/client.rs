#![allow(dead_code)]
use std::{
    collections::HashMap,
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::protocol::{
    Deserializer, Entry, Request, Response, RetrieveRequest, RetrieveResponse, StoreRequest,
    StoreResponse,
};

pub struct Client {
    socket: TcpStream,
    store: Arc<Mutex<HashMap<String, Entry>>>,
}

impl Client {
    pub fn new(socket: TcpStream, store: Arc<Mutex<HashMap<String, Entry>>>) -> Self {
        Self { socket, store }
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
    pub fn set(&mut self, data: Entry) {
        let key = data.key.clone();
        self.get_store().insert(key, data);
    }
    pub fn update_value<F>(&mut self, key: &String, f: F)
    where
        F: FnOnce(&mut Entry),
    {
        if let Some(e) = self.get_store().get_mut(key) {
            f(e);
        };
    }

    pub fn add(&mut self, data: Entry) {
        if !self.get_store().contains_key(&data.key) {
            self.set(data);
        };
    }

    pub fn replace(&mut self, data: Entry) {
        self.update_value(&data.key, |e: &mut Entry| {
            e.value = data.value;
            e.len = data.len
        });
    }

    pub fn append(&mut self, data: Entry) {
        self.update_value(&data.key, |e: &mut Entry| {
            e.value += &data.value;
            e.len += data.len;
        });
    }
    pub fn prepend(&mut self, data: Entry) {
        self.update_value(&data.key, |e: &mut Entry| {
            e.value = data.value + &e.value;
            e.len += data.len;
        });
    }

    pub fn get(&self, key: &String) -> Entry {
        let lock = self.get_store();
        let value = lock.get(key).unwrap();
        return value.clone();
    }

    pub fn handle_connection(&mut self) {
        let mut buf = [0u8; 512];
        let input = self.read(&mut buf).unwrap();
        let mut d = Deserializer::from_string(input);
        let request = d.deserialize();
        println!("request: {:?}", request);
        self.handle_request(request);
    }
    fn handle_request(&mut self, request: Request) -> Response {
        match request {
            Request::Store(req) => Response::Store(self.handle_store(req)),
            Request::Retreive(req) => Response::Retrieve(self.handle_retrieve(req)),
            Request::FlushAll => self.handle_flush_all(),
            Request::Delete(_key) => todo!(),
        }
    }

    fn handle_store(&mut self, req: StoreRequest) -> StoreResponse {
        match req {
            StoreRequest::Set(e) => self.set(e),
            StoreRequest::Add(e) => self.add(e),
            StoreRequest::Replace(e) => self.replace(e),
            StoreRequest::Append(e) => self.append(e),
            StoreRequest::Prepend(e) => self.prepend(e),
        };
        println!("store: {:?}", self.get_store());
        return StoreResponse::Stored;
    }
    fn handle_retrieve(&self, req: RetrieveRequest) -> RetrieveResponse {
        match req {
            RetrieveRequest::Get(key) => {
                let value = self.get(&key);
                println!("value: {:?}", value);
                RetrieveResponse::Value(value)
            }
            RetrieveRequest::Gets(_) => todo!(),
        }
    }
    fn handle_flush_all(&mut self) -> Response {
        self.get_store().clear();
        return Response::End;
    }

    fn handle_delete(&mut self) {
        todo!()
    }
}
