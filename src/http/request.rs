use super::method::Method;
use super::method::MethodError;
use super::query_string::QueryString;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as fmtResult;
use std::str;
use std::str::Utf8Error;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Request<'input_buf_stream> {
    path: &'input_buf_stream str,
    query_string: Option<QueryString<'input_buf_stream>>,
    method: Method,
}

impl<'input_buf_stream> TryFrom<&'input_buf_stream [u8]> for Request<'input_buf_stream> {
    type Error = ParseError;

    fn try_from(buf: &'input_buf_stream [u8]) -> Result<Request<'input_buf_stream>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i]
        }

        return Ok(Self {
            path: path,
            query_string: query_string,
            method: method,
        });
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidMethod => "Invalid Method",
            ParseError::InvalidProtocol => "Invalid Protocol",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
