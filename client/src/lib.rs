#![allow(dead_code, unused)]
use parser::{Entry, Request, Response, RetrieveRequest, StoreRequest};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod message;

pub struct Client<'a> {
    addr: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    fn connect(&self) -> TcpStream {
        return TcpStream::connect(self.addr.clone()).unwrap();
    }
    fn send(&mut self, request: Request) -> Response {
        let mut stream = self.connect();
        stream.write(request.to_string().as_bytes()).unwrap();
        return self.read(stream);
    }
    fn store(&mut self, store_req: StoreRequest) -> Response {
        self.send(Request::Store(store_req))
    }
    fn read(&mut self, mut stream: TcpStream) -> Response {
        let mut buf = [0u8; 512];
        let n = stream.read(&mut buf).unwrap();
        if n == 0 {
            panic!();
        }
        return Response::from_string(std::str::from_utf8(&buf[..n]).unwrap());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut client = Client::new("localhost:9889");
        let e = Entry::new("hello".into(), "value".into(), 5);
        let res = client.set(e);
        println!("res: {:?}", res);
        let e2 = Entry::new("hello".into(), " world".into(), 6);
        let res = client.append(e2);
        println!("res: {:?}", res);
        // client.get("hello".into());
    }
}
