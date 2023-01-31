pub mod error;
pub mod store;
use crate::{deserializer::Deserializer, Entry};
use error::*;
use std::str::FromStr;
pub use store::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    Store(StoreResponse),
    Retrieve(Entry),
    Delete(DeleteResponse),
    Ok,
    End,
    Error(MemcachedError),
    // ServerError,
    // errors
    // send not stored error from the error
    // add to report that these things were implemented
    // InvalidKey,
    // CommandError,
    // ValueError,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}

impl DeleteResponse {
    pub fn to_string(&self) -> String {
        let s = match self {
            DeleteResponse::Deleted => "DELETED",
            DeleteResponse::NotFound => "NOT_FOUND",
        };
        return format!("{}\r\n", s);
    }
}

impl Response {
    pub fn to_string(&self) -> String {
        let s = match self {
            Response::Store(s) => s.to_string(),
            Response::Retrieve(e) => format!("VALUE {}", e.to_res_str()),
            Response::Error(e) => e.to_string(),
            Response::Delete(d) => d.to_string(),
            Response::Ok => format!("DELETE\r\n"),
            Response::End => format!(""),
        };

        return format!("{}END\r\n", s);
    }
}

impl FromStr for Response {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.split(" ").collect::<Vec<&str>>().len() == 1 {
            if &s[s.len() - 2..] != "\r\n" {
                return Err(());
            };
            match &s[..s.len() - 2] {
                // "END" => Self::End,
                "CLIENT_ERROR" => Ok(Self::Error(MemcachedError::ClientError)),
                "SERVER_ERROR" => Ok(Self::Error(MemcachedError::ServerError)),
                "ERROR" => Ok(Self::Error(MemcachedError::Error)),
                _ => Ok(Self::Store(StoreResponse::from_str(s).unwrap())),
            }
        } else {
            let idx = s.find(" ").unwrap();
            if !(&s[..idx] == "VALUE") {
                return Err(());
            }
            return Ok(Self::Retrieve(Entry::from_res_str(&s[idx + 1..])));
        }
    }
}
