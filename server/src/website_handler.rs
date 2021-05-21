use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Come on</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    
    }

}