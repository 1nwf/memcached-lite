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
const DELETE_COMMAND: &str = "delete";
const FLUSH_COMMAND: &str = "flush_all";

enum MessageType {
    Request(Request),
    Response(Response),
}
#[derive(Debug)]
pub enum Request {
    Store(StoreRequest),
    Retreive(RetrieveRequest),
    FlushAll,
    Delete(String),
}

impl Request {
    fn from_str(s: &str) -> Self {
        let cmd = &s[..s.find(" ").unwrap()];
        if STORE_COMMANDS.contains(&cmd) {
            return Request::Store(StoreRequest::from_str(s));
        } else if RETRIEVE_COMMANDS.contains(&cmd) {
            return Request::Retreive(RetrieveRequest::from_str(s));
        } else if cmd == DELETE_COMMAND {
            let req = s
                .split(" ")
                .filter(|e| !e.is_empty())
                .map(|e| e.trim())
                .collect::<Vec<&str>>();
            if req.len() != 2 {
                panic!("invalid cmd")
            }
            return Request::Delete(req[1].to_string());
        } else if cmd == FLUSH_COMMAND {
            todo!();
        }

        panic!();
    }
}
pub enum Response {
    Store(StoreResponse),
    Retrieve(RetrieveResponse),
    End,
    Error,
    ClientError,
    ServerError,
    // errors
    // send not stored error from the error
    // add to report that these things were implemented
    InvalidKey,
    CommandError,
    ValueError,
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

#[derive(Debug)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}

impl StoreRequest {
    fn get_cmd_from_str(s: &str) -> fn(Entry) -> StoreRequest {
        let k = match s {
            "set" => StoreRequest::Set,
            "add" => StoreRequest::Add,
            "replace" => StoreRequest::Replace,
            "append" => StoreRequest::Append,
            "prepend" => StoreRequest::Prepend,
            _ => panic!("invalid"),
        };
        return k;
    }
    fn from_str(s: &str) -> StoreRequest {
        let idx = s.find("\r\n").unwrap();
        let cmd = &s[..idx]
            .split(" ")
            .filter(|e| !e.is_empty())
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
        let value = &s[idx + 2..];
        if cmd.len() != 3 {
            panic!("invalid request");
        }
        let (cmd, key, size) = (cmd[0], cmd[1], cmd[2]);
        let request = Self::get_cmd_from_str(cmd);
        let size = size.parse::<u32>().unwrap();
        let value = value[..size as usize].to_string();
        let entry = Entry::new(key.to_string(), value, size);
        return request(entry);
    }
}

impl RetrieveRequest {
    fn get_cmd_from_str(s: &str) -> fn(String) -> RetrieveRequest {
        let k = match s {
            "get" => RetrieveRequest::Get,
            "gets" => RetrieveRequest::Gets,
            _ => panic!("invalid"),
        };
        return k;
    }
    fn from_str(s: &str) -> RetrieveRequest {
        let request = s
            .split(" ")
            .filter(|e| !e.is_empty())
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
        if request.len() != 2 {
            panic!("InvalidRequest")
        }
        return Self::get_cmd_from_str(request[0])(request[1].to_string());
    }
}

pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}
pub enum RetrieveResponse {
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

    pub fn deserialize(&mut self) -> Request {
        let cmd = &self.input[..self.input.find(" ").unwrap()];
        if STORE_COMMANDS.contains(&cmd) {
            return Request::Store(StoreRequest::from_str(self.input));
        } else if RETRIEVE_COMMANDS.contains(&cmd) {
            return Request::Retreive(RetrieveRequest::from_str(self.input));
        } else if cmd == DELETE_COMMAND {
        } else if cmd == FLUSH_COMMAND {
        };
        panic!("invalid command");
    }
}
