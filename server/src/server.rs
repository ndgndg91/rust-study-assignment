use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
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
                                    Response::new(StatusCode::Ok, Some("<h1>Sex</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to parse {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse {}", e);
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
