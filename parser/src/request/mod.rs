pub mod store;
pub use store::*;

use crate::error::MemcachedError;

#[derive(Debug)]
pub enum Request {
    Store(StoreRequest),
    Retreive(RetrieveRequest),
    FlushAll,
    Delete(String),
}

impl Request {
    pub fn from_str(s: &str) -> Result<Self, MemcachedError> {
        let idx = s.find(" ");
        if let Some(idx) = idx {
            let cmd = &s[..idx];
            if STORE_COMMANDS.contains(&cmd) {
                return Ok(Request::Store(StoreRequest::from_str(s)?));
            } else if RETRIEVE_COMMANDS.contains(&cmd) {
                return Ok(Request::Retreive(RetrieveRequest::from_str(s)));
            } else if cmd == DELETE_COMMAND {
                let req = s
                    .split(" ")
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>();
                if req.len() != 2 {
                    return Err(MemcachedError::Error);
                }
                let key = req[1].replace("\r\n", "");
                return Ok(Request::Delete(key));
            }
        }
        let cmd = s
            .split(' ')
            .filter(|e| !e.is_empty())
            .collect::<Vec<&str>>();
        if cmd.len() <= 2 && cmd[0] == FLUSH_COMMAND && cmd[1] == "\r\n"
            || cmd[0] == (FLUSH_COMMAND.to_owned() + "\r\n")
        {
            return Ok(Request::FlushAll);
        }

        panic!("invalid command");
    }
    pub fn to_string(&self) -> String {
        match self {
            Request::Store(s) => s.to_string(),
            Request::Retreive(r) => r.to_string(),
            Request::FlushAll => "flush_all\r\n".into(),
            Request::Delete(key) => format!("delete {}\r\n", key),
        }
    }
}

#[derive(Debug)]
pub enum RetrieveRequest {
    Get(String),
    // Gets(String),
}

impl RetrieveRequest {
    fn get_cmd_from_str(s: &str) -> fn(String) -> RetrieveRequest {
        let k = match s {
            "get" => RetrieveRequest::Get,
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
    fn to_string(&self) -> String {
        match self {
            RetrieveRequest::Get(key) => format!("get {}\r\n", key),
        }
    }
}
