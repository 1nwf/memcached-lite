use std::str::FromStr;

use crate::deserializer::Deserializer;

const ERROR: &str = "ERROR";
const CLIENT_ERROR: &str = "CLIENT_ERROR";
const SERVER_ERROR: &str = "SERVER_ERROR";
#[derive(Debug, PartialEq, Eq)]
pub enum MemcachedError {
    /// invalid command error
    Error,
    /// the client does not conform to the protocol.
    ClientError,
    /// an error that prevents the server from carrying on the request
    ServerError,
}

impl MemcachedError {
    pub fn to_string(&self) -> String {
        let s = match self {
            MemcachedError::Error => ERROR,
            MemcachedError::ClientError => SERVER_ERROR,
            MemcachedError::ServerError => CLIENT_ERROR,
        };
        return format!("{}\r\n", s);
    }
}

impl FromStr for MemcachedError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("unable to parse string".into());
        }
        let mut d = Deserializer::from_str(s);
        let line = d.next_line()?;
        match line {
            ERROR => Ok(MemcachedError::Error),
            CLIENT_ERROR => Ok(MemcachedError::ClientError),
            SERVER_ERROR => Ok(MemcachedError::ServerError),
            _ => Err("invalid error".into()),
        }
    }
}
