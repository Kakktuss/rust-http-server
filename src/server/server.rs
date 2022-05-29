use std::collections::HashMap;
use std::error::Error;
use std::str::{from_utf8, FromStr};
use crate::server::config;
use crate::server::connection::connection::{ServerError, Connection, ServerConnectionError, ServerInternalError};
use crate::server::http::request::{HttpMethod, HttpRequest};
use crate::server::http::result::{HttpResult, HttpResultError, HttpStatusCode};
use crate::server::http::result::HttpStatusCode::ServerError;

pub struct Server<T>
    where T: Connection
{
    config: config::Config,
    connection: T,
    handlers: HashMap<HttpMethod, Box<dyn Fn(HttpRequest) -> Result<HttpResult, HttpResultError>>>
}

impl<T: Connection> Server<T>
{

    pub fn new(config: config::Config, connection: T) -> Box<Server<T>>
    {
        Box::from(Server {
            config,
            connection,
            handlers: HashMap::new()
        })
    }

    pub fn run(&self)
    {
        self.connection.listen(|request| Self::handle_request(request));
    }

    fn handle_request(request: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>
    {
        from_utf8(request)
            .map_or_else(
                |_| {
                    Err(ServerConnectionError::new(""))
                },
            |request| {
                Ok(HttpRequest::from_str(request))
            })
            .map_or_else(
                |e| {
                    Err(e.into())
                },
                |request|
                match request {
                    Ok(request) => Ok(request),
                    Err(err) => Err(err.into())
                }
            )
            .map_or_else(
                |err|  {
                    Err(err)
                },
                move |request| {
                    let method_handler = Self::get_handler(&request.method);

                    match method_handler
                    {
                        Some(handler) => {
                            let result = (**handler)(request);

                            match result
                            {
                                Ok(result_value) => Ok(result_value.to_string().into_bytes()),
                                Err(e) => Err(e.into())
                            }
                        },
                        None => Err(Box::from(ServerInternalError::new("Unable to find handler for this method")))
                    }
                }
            )
    }

    fn get_handler(&self, method: &HttpMethod) -> Option<&Box<dyn Fn(HttpRequest) -> Result<HttpResult, HttpResultError>>>
    {
        self.handlers.get(method)
    }

    pub fn register_handler<H: 'static + Copy + Fn(HttpRequest) -> Result<HttpResult, HttpResultError>>(&mut self, method: HttpMethod, handler: H)
    {
        self.handlers.insert(method, Box::new(handler));
    }
}