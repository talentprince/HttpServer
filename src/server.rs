use std::{io::Read, net::TcpListener};
use crate::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening to {}", self.addr);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Recieved something: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse request [{}]", e),
                            };
                        },
                        Err(e) => println!("Failed to read from socket. [{}]", e),
                    };
                }
                Err(e) => {
                    println!("Failed to connect. [{}]", e);
                }
            }
        }
    }
}
