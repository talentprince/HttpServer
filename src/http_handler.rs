use std::fs;

use super::{server::Handler, http::{Request, Response, StatusCode, Method}};
pub struct WebRequestHandler {
    public_path: String,
}

impl WebRequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_name: &str) -> Option<String> {
        let file_path = format!("{}/{}", self.public_path, file_name);
        match fs::canonicalize(file_path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Attemp to browser illegal path {}", file_name);
                    None
                }
            }
            Err(_) => None,
        }
        
    }
}

impl Handler for WebRequestHandler {
    fn handle_response(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/welcome" => Response::new(StatusCode::OK, self.read_file("welcome.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::OK, Some(content)),
                    None => Response::new(StatusCode::NotFound, self.read_file("404.html"))
                }
                _ => Response::new(StatusCode::NotFound, self.read_file("404.html"))
            },
            _ => Response::new(StatusCode::NotFound, self.read_file("404.html"))
        }
        
    }
}