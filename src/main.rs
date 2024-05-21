use std::{
    io::{Read, Write},
    net::TcpListener,
};

const CLRF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => loop {
                let mut incoming = [0; 1024];

                match _stream.read(&mut incoming) {
                    Ok(n) => {
                        if n <= 0 {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("error reading from stream: {}", e);
                    }
                }

                let buffer = format!("+PONG{CLRF}");

                match _stream.write(buffer.as_bytes()) {
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
    }
}
