#![allow(dead_code)]
mod deserializer;
mod request;
mod response;
use std::time::Duration;

pub use request::*;
pub use response::*;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub key: String,
    pub flags: u32,
    pub exptime: u32, // can't be longer than 60*60*24*30 seconds (30 days)
    pub len: u32,
    pub value: String,
}
impl Entry {
    pub fn new(key: String, flags: u32, exptime: u32, value: String, len: u32) -> Self {
        if exptime > 60 * 60 * 24 * 30 {
            panic!("exptime can't be greater than 30 days");
        }
        Self {
            key,
            flags,
            exptime,
            len,
            value,
        }
    }
    pub fn default_new(key: String, value: String, len: u32) -> Self {
        Self::new(key, 0, 0, value, len)
    }
    pub fn to_string(&self, exp: bool) -> String {
        let Self {
            key,
            flags,
            exptime,
            value,
            len,
        } = self;
        let mut s = format!("{} {}", key, flags);
        if exp {
            s = format!("{} {}", s, exptime);
        }
        return format!("{} {}\r\n{}\r\n", s, len, value);
    }
    pub fn to_req_str(&self) -> String {
        return self.to_string(true);
    }

    pub fn to_res_str(&self) -> String {
        return self.to_string(false);
    }

    pub fn from_res_str(s: &str) -> Self {
        return Self::from_string(s, false);
    }

    pub fn from_req_str(s: &str) -> Self {
        return Self::from_string(s, true);
    }
    pub fn from_string(s: &str, exp: bool) -> Self {
        let idx = s.find("\r\n").unwrap();
        let v = &s[..idx]
            .split(" ")
            .filter(|e| !e.is_empty())
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
        if (exp && v.len() != 4) || (!exp && v.len() != 3) {
            panic!("invalid entry");
        }

        let value = &s[idx + 2..];
        if exp {
            let (key, flags, exptime, size) = (v[0], v[1], v[2], v[3]);
            let flags = flags.parse::<u32>().unwrap();
            let exptime = exptime.parse::<u32>().unwrap();
            let size = size.parse::<u32>().unwrap();
            let value = value[..size as usize].to_string();
            return Entry::new(key.to_string(), flags, exptime, value, size);
        } else {
            let (key, flags, size) = (v[0], v[1], v[2]);
            let flags = flags.parse::<u32>().unwrap();
            let size = size.parse::<u32>().unwrap();
            let value = value[..size as usize].to_string();
            return Entry::new(key.to_string(), flags, 0, value, size);
        }
    }

    pub fn append(&mut self, e: &Entry) {
        self.value += &e.value;
        self.len += e.len;
    }
    pub fn prepend(&mut self, e: &Entry) {
        self.value = format!("{}{}", e.value, self.value);
        self.len += e.len;
    }
    pub fn replace(&mut self, e: &Entry) {
        _ = std::mem::replace(self, e.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
