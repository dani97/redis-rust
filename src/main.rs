#![allow(unused_imports)]
use std::{
    collections::HashSet, fs::{self, read}, hash::Hash, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, process::Command, thread::{self, JoinHandle}
};

fn handle_connection(mut stream: TcpStream) {
    // let mut buf_reader = BufReader::new(&mut stream);
    // let mut line = String::new();
    // let mut no_of_bytes = 0;
    // let mut commands: Vec<String> = Vec::new();
    // buf_reader.read_line(&mut line).unwrap();
    
    // for _ in 0..6 {
    //     line.clear();
    //     no_of_bytes = buf_reader.read_line(&mut line).unwrap();
    //     line = line.trim().to_owned();
    //     println!("{}", line);
    //     println!("{}", no_of_bytes);
    //     if let Some(command) = commands_list.get(&line) {
    //         commands.push(command.to_string());
    //     }
    // }

    //println!("{:?}", commands);
    let mut buf = [0; 512];
    loop {
        
        let no_of_bytes = stream.read(&mut buf).unwrap();
        if no_of_bytes == 0 {
            break;
        }
        stream.write(b"+PONG\r\n").unwrap();
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
            Ok(mut stream) => {
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
