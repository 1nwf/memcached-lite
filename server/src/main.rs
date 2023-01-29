use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::client::Client;
use parser::Entry;

mod client;
fn main() {
    let addr = "127.0.0.1:9889";
    let listener = TcpListener::bind(addr).expect("port 9889 in use");
    let store: Arc<Mutex<HashMap<String, Entry>>> = Arc::new(Mutex::new(HashMap::new()));
    loop {
        match listener.accept() {
            Ok((socket, _)) => {
                println!("client connected");
                let k: Arc<Mutex<HashMap<String, Entry>>> = Arc::clone(&store);
                thread::spawn(|| {
                    let mut client = Client::new(socket, k);
                    client.handle_connection();
                });
            }
            Err(e) => println!("failed to connect to client {e}"),
        }
    }
}
