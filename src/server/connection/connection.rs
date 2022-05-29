use std::fmt;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait ServerError: Error
{
    fn new(msg: &str) -> Self;
}

#[derive(Debug, Clone)]
pub struct ServerConnectionError {
    msg: String
}

impl Display for ServerConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ServerConnectionError {}

impl ServerError for ServerConnectionError
{
    fn new(msg: &str) -> ServerConnectionError
    {
        ServerConnectionError {
            msg: String::from(msg)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerInternalError
{
    msg: String
}

impl Display for ServerInternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ServerInternalError {}

impl ServerError for ServerInternalError
{
    fn new(msg: &str) -> ServerInternalError
    {
        ServerInternalError {
            msg: String::from(msg)
        }
    }
}

pub trait Connection
{
    fn listen<T: 'static + Copy + Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>> + Send + Sync>(
        &self,
        callback: T
    );
}