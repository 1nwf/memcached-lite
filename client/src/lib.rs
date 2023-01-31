#![allow(dead_code, unused)]
use protocol::{Entry, Request, Response, RetrieveRequest, StoreRequest};
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
                panic!("");
            } else if &buf[n_read - 5..n_read] == b"END\r\n" {
                n = n_read;
                break;
            }
        }
        let response_str = std::str::from_utf8(&buf[..n - 5]).unwrap();
        response_str.parse::<Response>().unwrap()
    }
    pub fn get(&mut self, key: String) -> Entry {
        let cmd = RetrieveRequest::Get(key);
        let request = Request::Retreive(cmd);
        if let Response::Retrieve(entry) = self.send(request) {
            return entry;
        };
        panic!("invalid response");
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
    use super::*;

    #[test]
    fn set_get() {
        let mut client = Client::new("localhost:9889");
        let key = "hello".to_string();
        let value = "hello".to_string();
        let len = value.len() as u32;

        let e = Entry::default_new(key.clone(), value.clone(), len);
        let res = client.set(e);
        assert_eq!(res, Response::Store(protocol::StoreResponse::Stored));

        let e2 = Entry::default_new(key.clone(), " world".into(), 6);
        let res = client.append(e2);

        assert_eq!(res, Response::Store(protocol::StoreResponse::Stored));

        let entry = client.get(key.clone());
        assert_eq!(
            entry,
            Entry::default_new("hello".into(), "hello world".into(), 11)
        );
    }

    #[test]
    fn append() {}

    #[test]
    fn prepend() {}

    #[test]
    fn replace() {
        let mut client = Client::new("localhost:9889");
        let key = "key1".to_string();
        let value = "value1".to_string();
        let len: u32 = value.len() as u32;
        let entry = Entry::default_new(key.clone(), value, len);

        client.set(entry.clone());

        let res = client.get(key.clone());
        assert_eq!(res, entry);

        let new_entry = Entry::default_new(key.clone(), "value replaced".into(), 14);
        client.replace(new_entry.clone());

        let res = client.get(key);
        assert_eq!(res, new_entry);
    }

    #[test]
    fn delete() {}

    #[test]
    fn flush_all() {}
}
