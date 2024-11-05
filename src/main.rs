#![allow(unused_imports)]
use std::{
    fs::{self, read},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, process::Command,
};

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut line = String::new();
    let mut no_of_bytes = 0;
    for _ in 1..4 {
        line.clear();
        no_of_bytes = buf_reader.read_line(&mut line).unwrap();
        println!("{}", line);
        println!("{}", no_of_bytes);
    }
    if line.eq("PING\r\n") {
        stream.write(b"+PONG\r\n").unwrap();
    } 
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
