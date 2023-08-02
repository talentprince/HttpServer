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
                            dbg!(&buf[..20]);
                            let result: Result<Request, _> = TryFrom::try_from(&buf[..]);
                            println!("Recieved something: {}", String::from_utf8_lossy(&buf))
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
