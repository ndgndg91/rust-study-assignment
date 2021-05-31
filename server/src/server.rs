use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;
use crate::http::website_handler::{WebSiteHandler, Handler};
use std::sync::{mpsc, Mutex, Arc};

struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads: workers, sender }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker { id, thread }
    }
}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        let pool = ThreadPool::new(10);

        for stream in listener.incoming() {
            println!("Received a request!");
            let stream = stream.unwrap();

            pool.execute(|| {
                let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
                let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
                println!("public path : {}", public_path);
                handle_connection(stream, WebSiteHandler::new(public_path));
            });
        }

        fn handle_connection(mut stream: TcpStream, mut handler: impl Handler) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            println!();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            let parse_result = Request::try_from(&buffer[..]);
            let response;
            if parse_result.is_ok() {
                let request = parse_result.unwrap();
                println!("{:?}", request);
                response = handler.handle_request(&request);
            } else {
                let err = parse_result.unwrap_err();
                print!("Failed to parse {}", err);
                response = handler.handle_bad_request(&err)
            }

            if let Err(e) = response.send(&mut stream) {
                println!("Failed to response {}", e);
            }
        }
    }
}
