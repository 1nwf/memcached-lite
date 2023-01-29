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
    fn new(key: String, value: String, len: u32) -> Self {
        Self { key, value, len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
