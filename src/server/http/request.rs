use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;
use crate::server::connection::connection::ServerError;

#[derive(Debug, Clone)]
pub struct HttpRequestError
{
    msg: String
}

impl Display for HttpRequestError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for HttpRequestError {}

impl ServerError for HttpRequestError
{
    fn new(msg: &str) -> Self {
        HttpRequestError {
            msg: String::from(msg)
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest
{
    pub method: HttpMethod,
}

impl FromStr for HttpRequest
{
    type Err = HttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
        {
            _ => Err(HttpRequestError::new("Unable to read http request"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET
}

impl FromStr for HttpMethod
{
    type Err = HttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
             _ => Err(HttpRequestError::new("Unknow http method"))
        }
    }
}