use std::{
    collections::HashMap,
    net::TcpListener,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use crate::client::Client;
mod data_file;
use data_file::Data;
use lazy_static::lazy_static;
use protocol::Entry;

const DATA_FILENAME: &str = "./data.txt";

lazy_static! {
    static ref STORE: Arc<Mutex<HashMap<String, Entry>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref DATA_FILE: Arc<Mutex<Data>> = {
        let path = Path::new(DATA_FILENAME);
        Arc::new(Mutex::new(Data::new(path)))
    };
}

mod client;

fn main() {
    let addr = "127.0.0.1:9889";
    let listener = TcpListener::bind(addr).expect("port 9889 in use");
    init_data();
    loop {
        match listener.accept() {
            Ok((socket, _)) => {
                println!("client connected");
                let store: Arc<Mutex<HashMap<String, Entry>>> = Arc::clone(&STORE);
                let file = Arc::clone(&DATA_FILE);
                thread::spawn(|| {
                    let mut client = Client::new(socket, store, file);
                    client.handle_connection();
                });
            }
            Err(e) => println!("failed to connect to client {e}"),
        }
    }
}

fn init_data() {
    let data = DATA_FILE.lock().unwrap().read_data();
    let mut s = STORE.lock().unwrap();
    s.extend(data);
}
