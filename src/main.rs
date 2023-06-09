use server::Server;
use http::Method;
use http::Requst;

mod server;
mod http;

fn main() {
    let ip = String::from("127.0.0.1:8080");
    let server = Server::new(ip);
    server.run();
}
