use super::http::{ParseError, Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_response(&mut self, request: &Request) -> Response;
    fn handle_error(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(
            StatusCode::BadRequest,
            Some("<h1>Bad Request</h1>".to_string()),
        )
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
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
                                    handler.handle_response(&request)
                                }
                                Err(e) => {
                                    handler.handle_error(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
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
