use clap::Parser;
use rust_uri::Uri;
use std::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str::FromStr,
};

#[derive(Debug, Parser)]
struct Arguments {
    url: String,
}

fn main() {
    const BUF_SIZE: usize = 4096;

    let args = Arguments::parse();
    let uri = Uri::from_str(&args.url).unwrap();

    let hostname = uri.hostname;
    let path = uri.path;
    let addr = match uri.port {
        Some(port) => format!("{}:{}", &hostname, port),
        None => hostname.clone(),
    };

    let mut stream = TcpStream::connect(addr).unwrap();

    let req = format!(
        "\
        GET {path} HTTP/1.1\r\n\
        Host: {hostname}\r\n\
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
    println!();
}


#[test]
fn test() {
    let path = "/index.html";
    let hostname = "www.example.com";
    let req = format!(
        "\
        GET {path} HTTP/1.1\r\n\
        Host: {hostname}\r\n\
        \r\n"
        // r#"
        // GET {path} HTTP/1.1\r\n\
        // Host: {hostname}\r\n\
        // "#
    );

    println!(">>> {req}");
}