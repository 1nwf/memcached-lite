#![allow(dead_code)]
mod request;
mod response;
pub use request::*;
pub use response::*;

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub value: String,
    pub len: u32,
}
impl Entry {
    pub fn new(key: String, value: String, len: u32) -> Self {
        Self { key, value, len }
    }
    pub fn to_string(&self) -> String {
        let Self { key, value, len } = self;
        return format!("{} {}\r\n{}\r\n", key, len, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
