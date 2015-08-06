use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::String;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => { println!("Connection Failed! Error: {}", e); }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    stream.write(b"Hello World\r\n").unwrap();
}

pub fn start_client() {
    println!("Starting client!");
    let mut stream =  TcpStream::connect("127.0.0.1:9123").unwrap();
    loop {
        let mut string = String::new();
        match stream.read_to_string(&mut string) {
            Ok(i) => if i > 0 { println!("Read {} bytes: {}", i, string); },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
