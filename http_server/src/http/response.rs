use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code: status_code,
            body: body,
        }
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body: &str = match &self.body {
            Some(body_str) => body_str,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            &self.status_code,
            &self.status_code.reason_phrase(),
            body
        )
    }
}
