use super::server::Handler;
use super::http::{Request, Response, StatusCode};

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
    }

}