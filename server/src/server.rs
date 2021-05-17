use std::net::TcpListener;

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
            let (stream, addr) = match listener.accept() {
                Ok((stream, addr)) => {
                    (stream, addr)
                },
                Err(err) => println!("Failed to establish a connection : {}", err)
            };
        }
    }
}
