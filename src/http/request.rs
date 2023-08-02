use std::{
    error::Error,
    fmt::{Debug, Display},
    str::Utf8Error,
    result,
    str
};

use super::method::{Method, MethodError};
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<&'buf str>,
    method: Method,
}

impl <'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let result = str::from_utf8(value)?;
        let (method, result) = read_next_word(result).ok_or(ParseError::InvalidReqest)?;
        let (protocal, result) = read_next_word(result).ok_or(ParseError::InvalidReqest)?;
        if protocal != "Http/1.1" {
            return Err(ParseError::InvalidProtocal);
        }
        
        let (mut path  , result) = read_next_word(result).ok_or(ParseError::InvalidReqest)?;
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }
        return Ok(Self { 
            path: path,
            query: query_string,
            method: method,
         });    
    }
}

fn read_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\n' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    return None;
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidReqest
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

pub enum ParseError {
    InvalidReqest,
    InvalidMethod,
    InvalidProtocal,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidReqest => "Invalid Reqest",
            ParseError::InvalidMethod => "Invalid Method",
            ParseError::InvalidProtocal => "Invalid Protocal",
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
