use crate::http::Request;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address: address }
    }
    pub fn run(self) {
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
                    match Request::try_from(&buffer[..]) {
                        Ok(request) => {}
                        Err(e) => {
                            println!("Failed to parse request {}", e);
                        }
                    };
                }
                Err(_) => {
                    println!("Request could not be read!");
                }
            }
        }
    }
}