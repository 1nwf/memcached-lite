use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

mod client;
mod protocol;
fn main() {
    let addr = "127.0.0.1:9889";
    let listener = TcpListener::bind(addr).expect("port 9889 in use");
    let mut thread_handlers = vec![];
    loop {
        match listener.accept() {
            Ok((socket, _)) => {
                thread_handlers.push(thread::spawn(|| handle_client(socket)));
            }
            Err(e) => println!("failed to connect to client {e}"),
        }
    }
}

fn handle_client(mut socket: TcpStream) {
    loop {
        let mut buf = [0u8; 100];
        let n = socket.read(&mut buf).unwrap();
        if n != 0 {
        } else {
            break;
        }
    }
}
