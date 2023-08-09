use std::{io::Read, net::TcpListener};
use crate::http::{Request, Response, StatusCode, response};

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
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::NotFound, Some("<h1>Ha, page not found</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, Some("<h1>Bad Request</h1>".to_string()))
                                },
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
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
