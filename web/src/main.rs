use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for line in listener.incoming() {
        let stream = line.unwrap();

        // println!("Connection established!");

        thread::spawn(|| {
            handle_con(stream);
        });
    }
}

fn handle_con(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // let http_request: Vec<_> =
    //     buf_reader.lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    //
    // println!("Request: {http_request:#?}");

    println!("sleep start");
    thread::sleep(Duration::from_secs(5));
    println!("sleep end");

    let line = buf_reader.lines().next().unwrap().unwrap();

    let response;

    if line != "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    } else {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    }

    stream.write_all(response.as_bytes()).unwrap();
}
