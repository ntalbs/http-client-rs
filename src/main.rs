use std::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};
fn main() {
    const BUF_SIZE: usize = 4096;
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    let host = "localhost";
    let path = "/hello/world?query=100";

    let req = format!("\
        GET {path} HTTP/1.1\r\n\
        Host: {host}\r\n\
        \r\n"
    );

    println!("{}", req);

    let _ = stream.write(req.as_bytes());
    let _ = stream.flush();

    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    loop {
        let nread = stream.read(&mut buf).unwrap();
        if nread == 0 {
            break;
        }
        print!("{}", str::from_utf8(&buf).unwrap());
    }
    println!("");
}