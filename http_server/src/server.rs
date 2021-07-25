use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address: address }
    }
    pub fn run(self, mut handler: impl Handler) {
        let listener: TcpListener = match TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(e) => {
                panic!(
                    "Could not bind {}, the resulting error is: \n{:?}",
                    self.address, e
                );
            }
        };
        println!("The server is listening at {}.", self.address);

        loop {
            let (mut stream, socket_addr): (TcpStream, SocketAddr) = match listener.accept() {
                Ok((mut stream, socker_addr)) => (stream, socker_addr),
                Err(_) => continue,
            };
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received request: {}.", String::from_utf8_lossy(&buffer));
                    let response = match Request::try_from(&buffer[..]) {
                        Ok(request) => handler.handle_request(&request),
                        Err(e) => handler.handle_bad_request(&e),
                    };
                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {}", e);
                    }
                }
                Err(_) => {
                    println!("Request could not be read!");
                }
            }
        }
    }
}
