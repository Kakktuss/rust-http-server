use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use serde_json::Value;
use crate::server::config::{Config, ConfigBuilder};
use crate::server::connection::connection::Connection;
use crate::server::connection::tcp_connection::TcpServerConnection;
use crate::server::server::Server;

mod server;

fn main() {

    let tcp_connection = match TcpServerConnection::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000))
    {
        Ok(tcpConnection) => tcpConnection,
        Err(e) => panic!("Can't build tcp connection")
    };

    let config = match ConfigBuilder::from_config("./config.json")
    {
        Ok(configBuilder) => configBuilder.build(),
        Err(e) => panic!("Can't build configuration")
    };

    let server = Server::<TcpServerConnection>::new(config, tcp_connection);

    server.run();

}
