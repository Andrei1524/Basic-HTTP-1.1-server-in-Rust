use super::method::{ Method, MethodError };
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString};

// 'buf -  lifetime for our buffer
// we derive so we can easily debug
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

// these are getters, because we cannot access request params outside this module
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    // this way we can change the type of the query string we dont need to return option
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    //
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request  = str::from_utf8(buf)?;

        // here we do var shadowing, get_next_word will find the first space return everything from left and return request again with whats left from the string,
        // request is being overwritten everytime like this:

        // method = GET, request = /search?name=abc&sort= HTTP/1.1
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // path = /search?name=abc&sort=1, request = HTTP/1.1\r\n...HEADERS...
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // this will still work, will unwrap automatically the i
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..])); // we add 1 because we dont want the ? to be captured, and + 1 is +1 byte (? is on byte in size)
            path = &path[..i];
        }

        // return
        Ok(Self { path, query_string, method })
    }
}

// here: -> (&str, &str)is a tuple, means we will return 2 string slices
// 'Option' means, we can also return 'none'
// here we try to find a space in str, if not, return None
fn get_next_word(request: &str) -> Option<(&str, &str)> {
   for (i, c) in request.chars().enumerate() {
       if c == ' '|| c == '\r' {
           return Some((&request[..i], &request[i + 1..])); // 1 is one byte, actually the space
       }
   }
   None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
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

impl ParseError {
    fn message(&self) -> &str {
        return match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
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

impl Error for ParseError {}
