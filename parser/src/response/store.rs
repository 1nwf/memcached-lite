use std::str::FromStr;

use crate::deserializer::Deserializer;
const STORED: &str = "STORED";
const NOT_STORED: &str = "NOT_STORED";
const EXISTS: &str = "EXISTS";
const NOT_FOUND: &str = "NOT_FOUND";

#[derive(Debug, PartialEq, Eq)]
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}

impl FromStr for StoreResponse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = Deserializer::from_str(s);
        let line = d.next_line()?.clone();
        match line {
            STORED => Ok(StoreResponse::Stored),
            NOT_STORED => Ok(StoreResponse::NotStored),
            EXISTS => Ok(StoreResponse::Exists),
            NOT_FOUND => Ok(StoreResponse::NotFound),
            _ => Err("invalid response".into()),
        }
    }
}

impl StoreResponse {
    pub fn to_string(&self) -> String {
        let s = match self {
            StoreResponse::Stored => "STORED",
            StoreResponse::NotStored => "NOT_STORED",
            StoreResponse::Exists => "EXISTS",
            StoreResponse::NotFound => "NOT_FUND",
        };
        return format!("{}\r\n", s);
    }
}