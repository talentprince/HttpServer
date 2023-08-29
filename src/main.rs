#![allow(dead_code)]
use std::env;

use http_handler::WebRequestHandler;
use server::Server;
mod http;
mod http_handler;
mod server;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let ip = String::from("127.0.0.1:8080");
    let server = Server::new(ip);
    server.run(WebRequestHandler::new(public_path));
}
