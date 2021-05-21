use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request : {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
 
        loop {            
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request!");
                            println!();
                            println!("{}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    println!("Failed to parse {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to response {}", e);
                            }
                        
                        },
                        Err(e) => println!("Failed to read from connection : {}", e)
                    };
                },
                Err(e) => println!("Failed to establish a connection : {}", e)
            };
        }
    }
}
