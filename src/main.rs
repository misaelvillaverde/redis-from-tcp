use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
};

mod lexer;
mod parser;
mod store;
mod token;

use lexer::*;
use parser::*;
use store::*;
use token::*;

const CLRF: &str = "\r\n";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let store = Arc::new(Mutex::new(Store::new()));

    for stream in listener.incoming() {
        let store = Arc::clone(&store);

        tokio::spawn(async move {
            match stream {
                Ok(mut _stream) => loop {
                    let mut request_buf = [0; 1024];

                    match _stream.read(&mut request_buf) {
                        Ok(n) => {
                            if n <= 0 {
                                break;
                            }
                        }
                        Err(e) => {
                            println!("error reading from stream: {}", e);
                        }
                    }

                    let request = String::from_utf8(request_buf.to_vec()).unwrap_or(String::new());

                    let tokens = tokenize(request);

                    let response = parse(&tokens, Arc::clone(&store));

                    match _stream.write(response.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("error writing to stream: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
