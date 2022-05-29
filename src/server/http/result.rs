use std::fmt::{Display, Formatter};
use crate::server::connection::connection::ServerError;

#[derive(Debug, Clone)]
pub struct HttpResultError
{
    msg: String
}

impl Display for HttpResultError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for HttpResultError {}

impl ServerError for HttpResultError
{
    fn new(msg: &str) -> Self {
        HttpResultError {
            msg: String::from(msg)
        }
    }
}


#[derive(Debug, Clone)]
pub struct HttpResult {
    status_code: HttpStatusCode,
    body: String
}

impl HttpResult
{
    pub fn new(status_code: HttpStatusCode, body: String) -> Self
    {
        HttpResult {
            status_code,
            body
        }
    }
}

impl ToString for HttpResult
{
    fn to_string(&self) -> String {


        String::from("")
    }
}

#[derive(Debug, Clone)]
pub enum HttpStatusCode {
    OK,
    NotFound,
    ServerError,
    ImATeapot
}