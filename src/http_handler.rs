use super::{server::Handler, http::{Request, Response, StatusCode}};
pub struct WebRequestHandler;

impl Handler for WebRequestHandler {
    fn handle_response(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(StatusCode::NotFound, Some("<h1>Ha, page not found</h1>".to_string()))
    }
}