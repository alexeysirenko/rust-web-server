use std::{
    fs,
    io::{BufRead, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let host = "127.0.0.1:7878";
    let listener = TcpListener::bind(host).unwrap();
    println!("Server is listening on {host}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_ref() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length {length}\r\n\r\n{contents}");

    match stream.write_all(response.as_bytes()) {
        Ok(success) => println!("Responded with: {success:#?}"),
        Err(error) => println!("Failed to respond: {error:#?}"),
    }
}
