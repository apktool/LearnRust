use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buffer = BufReader::new(&mut stream);
        let request_line = buffer.lines().next().unwrap().unwrap();
        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();
            let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        } else if request_line == "POST / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = "{\"message\": \"OK\"}";
            let length = contents.len();
            let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
