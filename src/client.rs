use std::{collections::HashMap, net::TcpStream};

struct Client {
    socket: TcpStream,
    store: HashMap<String, String>,
}

impl Client {
    fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            store: HashMap::new(),
        }
    }

    pub fn read(&mut self) {}

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        return self.store.get(&key);
    }
}
