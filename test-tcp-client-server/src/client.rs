use std::net::TcpStream;
use std::io::prelude::*;

pub fn connect() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");
    write!(stream,"hello from the client!\n").expect("write failed");
}