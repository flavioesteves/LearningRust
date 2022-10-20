use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
//use std::fmt::Debug;
//use std::fmt::Display;
//use std::fmt::Formatter;
//use std::fmt::Result as FmtResult;

pub struct Request_44 {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request_44 {
    type Error = ParseError;

    // Example of Request
    // GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //   let _string = String::from("asd");
        //   _string.encrypt();
        //   buf.encrypt();

        //L40 solution 1
        //match str::from_utf8(buf) {
        //    Ok(request) => {}
        //    Err(_) => return Err(ParseError::InvalidEncoding),
        //}

        //L40 Solution 2 normal used pattern
        //match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //    Ok(request) => {}
        //    Err(e) => return Err(e),
        //};

        // Special syntax that was created for solution 2
        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;

        // L42
        //match get_next_word(request) {
        //    Some((method, request)) => {}
        //    None => return Err(ParseError::InvalidRequest),
        //}
        //L42
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        //L44
        /*
                let mut query_string = None;
                match path.find('?') {
                    Some(i) => {
                        query_string = Some(&path[i + 1..]);
                        path = &path[..1];
                    }
                    None => {}
                };

                //L44
                let q = path.find('?');
                if q.is_some() {
                    let i = q.unwrap();
                    query_string = Some(&path[i + 1..]);
                    path = &path[..1];
                }
        */
        //L44
        if let Some(i) = path.find('?') {
            query_string = Some(path[i + 1..].to_string());
            path = &path[..1];
        }

        Ok(Self {
            path: path.to_string(),
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //L41
    //let mut iter = request.chars();
    //loop {
    //    let item = iter.next();
    //    match item {
    //        Some(c) => {}
    //        none => break,
    //    }
    //}

    //L41 for loop
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError_44 {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError_44 {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}

/*
trait Encrypt {
    fn encrypt(&self) -> Self;
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}
*/
