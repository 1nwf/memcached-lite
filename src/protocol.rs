#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub value: String,
    pub len: u32,
}
impl Entry {
    fn new(key: String, value: String, len: u32) -> Self {
        Self { key, value, len }
    }
}

const STORE_COMMANDS: [&str; 5] = ["set", "add", "replace", "append", "prepend"];
const RETRIEVE_COMMANDS: [&str; 2] = ["get", "gets"];
enum MessageType {
    Request(Request),
    Response(Response),
}
#[derive(Debug)]
pub enum Request {
    Store(StoreRequest),
    Retreive(RetrieveRequest),
    FlushAll,
}
pub enum Response {
    Store(StoreResponse),
    Retrieve(RetrieveResponse),
}

#[derive(Debug)]
pub enum StoreRequest {
    Set(Entry),
    Add(Entry),
    Replace(Entry),
    Append(Entry),
    Prepend(Entry),
}

#[derive(Debug)]
pub enum RetrieveRequest {
    Get(String),
    Gets(String),
}

pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}
pub enum RetrieveResponse {
    End,
    Value(Entry),
}

pub struct Message {
    msg: MessageType,
}

impl Message {
    fn new() -> Self {
        todo!();
    }
}

pub struct Deserializer<'a> {
    pub input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input: input.trim_matches(|v| v == ' '),
        }
    }
    pub fn split_request(&mut self) -> (&str, &str) {
        if let Some(idx) = self.input.find("\r\n") {
            let req = &self.input[..idx];
            self.input = &self.input[idx + 2..];
            return (req, self.input);
        } else {
            panic!("invalid request");
        }
    }

    pub fn deserialize(&mut self) -> Request {
        let (r, v) = self.split_request();
        let req = r.split(" ").collect::<Vec<&str>>();
        if req.len() < 2 {
            panic!("invalid request");
        }
        let cmd = req[0];
        if STORE_COMMANDS.contains(&cmd) {
            if req.len() < 3 {
                panic!("not enough arguments");
            }
            let entry = {
                let key = req[1];
                let size = req[2].parse::<u32>().unwrap();
                let last = v.get(size as usize..size as usize + 2).unwrap();
                if last != "\r\n" {
                    panic!("invalid value");
                }
                let value = v.get(0..size as usize).unwrap();
                Entry::new(String::from(key), String::from(value), size)
            };
            println!("entry: {:?}", entry);
            let req = match cmd {
                "set" => StoreRequest::Set(entry),
                "add" => StoreRequest::Add(entry),
                "replace" => StoreRequest::Replace(entry),
                "append" => StoreRequest::Append(entry),
                "prepend" => StoreRequest::Prepend(entry),
                _ => panic!(),
            };
            return Request::Store(req);
        } else if RETRIEVE_COMMANDS.contains(&cmd) {
            if req.len() < 2 {
                panic!("not enough arguments");
            }
            let key = String::from(req[1]);
            let req = match cmd {
                "get" => RetrieveRequest::Get(key),
                "gets" => RetrieveRequest::Gets(key),
                _ => panic!(),
            };
            return Request::Retreive(req);
        };
        panic!("invalid command");
    }
}
