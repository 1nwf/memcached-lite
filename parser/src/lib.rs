#![allow(dead_code)]
mod request;
mod response;
pub use request::*;
pub use response::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn from_string(s: &str) -> Self {
        let idx = s.find("\r\n").unwrap();
        let v = &s[..idx]
            .split(" ")
            .filter(|e| !e.is_empty())
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
        if v.len() != 2 {
            panic!("invalid entry");
        }
        let value = &s[idx + 2..];
        let (key, size) = (v[0], v[1]);
        let size = size.parse::<u32>().unwrap();
        let value = value[..size as usize].to_string();
        return Entry::new(key.to_string(), value, size);
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
