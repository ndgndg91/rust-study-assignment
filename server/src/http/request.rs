use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

    // GET /path/paht?query=string&query=String HTTP/1.1
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethodError,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethodError => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buf)?;
        unimplemented!()
    }
}