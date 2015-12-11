use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::{File};
use std::path::{Path};
use std::thread;

pub fn run(html_file_path: String) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening for connections on port 8080");

    let path = Path::new(&html_file_path);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("Woops. Can't find file."),
    };

    let mut html = String::new();
    file.read_to_string(&mut html).ok();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let html = html.clone();

                thread::spawn(move || {
                    handle_client(stream, html)
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
    drop(listener);
}

fn handle_client(mut stream: TcpStream, text: String) {
    let response = response_for(text);
    stream.write(&response.into_bytes()).unwrap();
}

fn response_for(body: String) -> String {
    format!("HTTP/1.1 200 OK\r\n\
             Content-Type: text/html;\
             charset=utf-8\r\n\
             content-length: {}\r\n\r\n\
             {}", body.len(), body)
}
