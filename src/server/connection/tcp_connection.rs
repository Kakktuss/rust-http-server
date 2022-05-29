use std::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use crate::server::worker_pool::WorkerPool;
use crate::server::connection::connection::{ServerError, Connection, ServerConnectionError};

pub struct TcpServerConnection
{
    listener: TcpListener,
    worker_pool: WorkerPool
}

impl TcpServerConnection
{
    pub fn new(socket: SocketAddr) -> Result<TcpServerConnection, ServerConnectionError>
    {
        let listener = match TcpListener::bind(socket)
        {
            Ok(listener) => listener,
            Err(e) => return Err(ServerConnectionError::new("Unable to bind to socket"))
        };

        Ok(TcpServerConnection {
            listener,
            worker_pool: WorkerPool::new(4)
        })
    }
    
    fn handle_incoming_connection<Callback: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>> + Send + Sync,
        Stream: Read + Write>(
        request_handler_callback: Callback,
        stream: &mut Stream)
    {
        let mut input_buffer: [u8; 1024] = [0; 1024];

        match stream.read(&mut input_buffer)
        {
            Ok(_) => {
                match(request_handler_callback)(&input_buffer)
                    .map(|message| stream.write(&message))
                    .map(|_| stream.flush())
                {
                    Ok(_) => println!("Request was succesfully handled"),
                    Err(e) => println!("Error when handling request: {}", e)
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}

impl Connection for TcpServerConnection
{
    fn listen<T: 'static + Copy + Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>> + Send + Sync>(&self, callback: T) {
        for connection in self.listener.incoming()
        {
            match connection
            {
                Ok(mut socket) => {
                    self.worker_pool.execute(move || {
                        Self::handle_incoming_connection(&callback, &mut socket)
                    })
                },
                Err(e) => println!("Error when getting client: {:?}", e)
            }
        }
    }
}

