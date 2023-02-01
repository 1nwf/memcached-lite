#![allow(dead_code, unused)]
use protocol::{Entry, MemcachedError, Request, Response, RetrieveRequest, StoreRequest};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Client {
    conn: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        let conn = TcpStream::connect(addr).expect("unable to connect to server");
        Self { conn }
    }

    fn send(&mut self, request: Request) -> Response {
        self.conn.write(request.to_string().as_bytes()).unwrap();
        return self.read();
    }
    fn store(&mut self, store_req: StoreRequest) -> Response {
        self.send(Request::Store(store_req))
    }
    fn read(&mut self) -> Response {
        let mut buf = [0u8; 512];
        let mut n;
        loop {
            let n_read = self.conn.read(&mut buf).unwrap();
            if n_read == 0 {
                panic!("read 0 bytes");
            } else if &buf[n_read - 5..n_read] == b"END\r\n" {
                n = n_read;
                break;
            }
        }
        let response_str = std::str::from_utf8(&buf[..n - 5]).unwrap();
        response_str.parse::<Response>().unwrap()
    }
    pub fn get(&mut self, key: String) -> Result<Entry, MemcachedError> {
        let cmd = RetrieveRequest::Get(key);
        let request = Request::Retreive(cmd);
        let response = self.send(request);
        match response {
            Response::Retrieve(e) => Ok(e),
            Response::Error(e) => Err(e),
            _ => panic!("invalid response"),
        }
    }
    pub fn set(&mut self, entry: Entry) -> Response {
        let cmd = StoreRequest::Set(entry);
        self.store(cmd)
    }
    pub fn append(&mut self, entry: Entry) -> Response {
        let cmd = StoreRequest::Append(entry);
        self.store(cmd)
    }
    pub fn prepend(&mut self, entry: Entry) -> Response {
        let cmd = StoreRequest::Prepend(entry);
        self.store(cmd)
    }
    pub fn delete(&mut self, key: String) -> Response {
        let cmd = Request::Delete(key);
        self.send(cmd)
    }
    pub fn flush_all(&mut self) -> Response {
        self.send(Request::FlushAll)
    }
    pub fn replace(&mut self, entry: Entry) -> Response {
        let cmd = StoreRequest::Replace(entry);
        self.store(cmd)
    }
}

#[cfg(test)]
mod tests {
    use protocol::{DeleteResponse, StoreResponse};

    use super::*;
    const SERVER_ADDR: &str = "localhost:9889";

    #[test]
    fn set_get() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "hello".to_string();
        let value = "hello".to_string();
        let len = value.len() as u32;

        let e = Entry::default_new(key.clone(), value.clone(), len);
        let res = client.set(e);
        assert_eq!(res, Response::Store(protocol::StoreResponse::Stored));

        let e2 = Entry::default_new(key.clone(), " world".into(), 6);
        let res = client.append(e2);

        assert_eq!(res, Response::Store(protocol::StoreResponse::Stored));

        let entry = client.get(key.clone()).unwrap();
        assert_eq!(
            entry,
            Entry::default_new("hello".into(), "hello world".into(), 11)
        );
    }

    #[test]
    fn append() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "append_key".to_string();
        let value = "first".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);
        let store_res = client.set(entry.clone());
        assert_eq!(store_res, Response::Store(StoreResponse::Stored));

        let new_entry = Entry::default_new(key.clone(), "first ".into(), 6);
        let append_res = client.append(new_entry.clone());
        assert_eq!(append_res, Response::Store(StoreResponse::Stored));
    }

    #[test]
    fn prepend() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "prepend_key".to_string();
        let value = "second".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);
        let store_res = client.set(entry.clone());
        assert_eq!(store_res, Response::Store(StoreResponse::Stored));

        let new_entry = Entry::default_new(key.clone(), "first ".into(), 6);
        let prepend_res = client.prepend(new_entry.clone());
        assert_eq!(prepend_res, Response::Store(StoreResponse::Stored));
    }

    #[test]
    fn replace() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "key1".to_string();
        let value = "value1".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);
        client.set(entry.clone());
        let res = client.get(key.clone()).unwrap();
        assert_eq!(res, entry);

        let new_entry = Entry::default_new(key.clone(), "value replaced".into(), 14);
        client.replace(new_entry.clone());

        let res = client.get(key).unwrap();
        assert_eq!(res, new_entry);
    }

    #[test]
    fn delete() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "key_to_delete".to_string();
        let value = "v1".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);
        let res = client.set(entry.clone());
        assert_eq!(res, Response::Store(StoreResponse::Stored));

        let res = client.delete(key.clone());
        assert_eq!(res, Response::Delete(DeleteResponse::Deleted))
    }

    #[test]
    fn flush_all() {
        let mut client = Client::new(SERVER_ADDR);
        let res = client.flush_all();
        assert_eq!(res, Response::Ok);
    }

    #[test]
    fn invalid_key() {
        let mut client = Client::new(SERVER_ADDR);
        let key = "key1\n".to_string();
        let value = "value1".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);
        let res = client.set(entry.clone());
        assert_eq!(res, Response::Error(MemcachedError::Error));
    }
}
