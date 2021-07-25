use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&'buf self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request_string = std::str::from_utf8(buf)?;

        let (method, remaining_request_string) =
            get_next_word(request_string).ok_or(ParseError::InvalidRequest)?;
        let (mut path, remaining_request_string) =
            get_next_word(remaining_request_string).ok_or(ParseError::InvalidRequest)?;
        let (protocol, remaining_request_string) =
            get_next_word(remaining_request_string).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(idx) = path.find('?') {
            query_string = Some(QueryString::from(&path[idx + 1..]));
            path = &path[..idx];
        }

        Ok(Self {
            path: path,
            query_string: query_string,
            method: method,
        })
    }
}
fn get_next_word(string: &str) -> Option<(&str, &str)> {
    for (idx, character) in string.chars().enumerate() {
        if character == ' ' || character == '\r' {
            return Some((&string[..idx], &string[idx + 1..]));
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
        Self::InvalidMethod
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
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
