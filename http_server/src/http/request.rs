use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{Debug, Formatter, Result as FmtResult};
ust std::str::Utf8Error;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request_string = std::str::from_utf8(buf)?;
        
    let (method, remaining_request_string) = get_next_word(request_string).ok_or(ParseError::InvalidRequest)?;
    let (path, remaining_request_string) = get_next_word(remaining_request_string).ok_or(ParseError::InvalidRequest)?;
    let (protocol, remaining_request_string) = get_next_word(remaining_request_string).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1"{
        return Err(ParseError::InvalidProtocol)
    }
    
    Request{
        method: method,
        path:path,
        query_string
    }

    }
}

fn get_next_word(string: &str) -> Option<(&str, &str)>{
    for (idx, character) in string.chars().enumerate(){
        if c ==  " " | c == "\r" {
            Some((string[..idx], string[idx+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
