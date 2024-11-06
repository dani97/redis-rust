#![allow(unused_imports)]
use std::{
    collections::HashSet, fs::{self, read}, hash::Hash, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, process::Command, thread::{self, JoinHandle}
};

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut line = String::new();
        let no_of_bytes = buf_reader.read_line(&mut line).unwrap();
        if no_of_bytes == 0 {
            break;
        }
        let no_of_commands = line.chars().nth(1).unwrap().to_digit(10).unwrap();
        let mut echo_flag = false;
        let mut commands: Vec<String> = Vec::new();
        println!("{}", no_of_commands);
        for _ in 0..no_of_commands {
            line.clear();
            buf_reader.read_line(&mut line).unwrap();
            line.clear();
            buf_reader.read_line(&mut line).unwrap();
            line = line.trim().to_owned();
            commands.push(line.clone());
        }

        println!("{:?}", commands);
        for command in commands {
            println!("{}", command);
            if echo_flag {
                let mut response = String::new();
                response.push_str("+");
                response.push_str(&command);
                response.push_str("\r\n");
                println!("{}", response);
                stream.write(response.as_bytes()).unwrap();
                echo_flag = false;
            }
            if command.contains("PING") {
                stream.write(b"+PONG\r\n").unwrap();
            } else if command.contains("ECHO") {
                echo_flag = true;
            }
            if command.contains("COMMAND") {
                stream.write(b"$0\r\n\r\n").unwrap();
            }
        }
    }
}

fn main() {
    let mut commands: HashSet<String> = HashSet::new();
    commands.insert("PING".to_string());
    let mut handlers = Vec::new();
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(|| {
                    handle_connection(stream);
                });
                handlers.push(handle);
                
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    for handle in handlers {
        handle.join().unwrap();
    }
}
