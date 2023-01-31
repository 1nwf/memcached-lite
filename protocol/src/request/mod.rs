use crate::{deserializer::Deserializer, Entry, MemcachedError};
mod store;
pub use store::*;

#[derive(Debug)]
pub enum Request {
    Store(StoreRequest),
    Retreive(RetrieveRequest),
    FlushAll,
    Delete(String),
}

impl Request {
    pub fn from_str(s: &str) -> Result<Self, MemcachedError> {
        let d = Deserializer::from_str(s);
        let line = d.next_line();
        if let Ok(line) = line {
            let words = Deserializer::split_words(line);
            // request is one line (GET, DELETE, FLUSH)
            if d.is_empty() {
                let cmd = words[0];
                if RETRIEVE_COMMANDS.contains(&cmd) {
                    return Ok(Request::Retreive(RetrieveRequest::from_str(s)?));
                } else if cmd == DELETE_COMMAND {
                    if words.len() == 2 {
                        return Ok(Request::Delete(words[1].to_string()));
                    }
                } else if cmd == FLUSH_COMMAND {
                    if words.len() == 1 {
                        return Ok(Request::FlushAll);
                    }
                };
                return Err(MemcachedError::ClientError);
            } else {
                return Ok(Request::Store(StoreRequest::from_str(s)?));
            }
        } else {
            return Err(MemcachedError::ClientError);
        }
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
    fn get_cmd_from_str(s: &str) -> Result<fn(String) -> RetrieveRequest, MemcachedError> {
        match s {
            "get" => Ok(RetrieveRequest::Get),
            _ => Err(MemcachedError::Error),
        }
    }
    fn from_str(s: &str) -> Result<RetrieveRequest, MemcachedError> {
        let d = Deserializer::from_str(s);
        let line = d.next_line();
        if let Ok(line) = line {
            let req = Deserializer::split_words(line);
            if !d.is_empty() || req.len() != 2 {
                return Err(MemcachedError::ClientError);
            }
            let cmd = Self::get_cmd_from_str(req[0])?;
            let key = req[1];
            if !Entry::is_valid_key(key) {
                return Err(MemcachedError::Error);
            }
            return Ok(cmd(req[1].to_string()));
        }
        return Err(MemcachedError::ClientError);
    }
    fn to_string(&self) -> String {
        match self {
            RetrieveRequest::Get(key) => format!("get {}\r\n", key),
        }
    }
}
