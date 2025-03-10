use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server running at http://127.0.0.1:3000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
