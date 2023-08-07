#![allow(dead_code)]

use http::Request;
use server::Server;

mod server;
mod http;

fn main() {
    let ip = String::from("127.0.0.1:8080");
    let server = Server::new(ip);
    server.run();
}
