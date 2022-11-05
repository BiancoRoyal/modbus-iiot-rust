use network::socket::parse_network_address;
use std::error::Error;
use std::io;
use std::net::{SocketAddr, TcpStream};

//	===============================================================================================

pub fn create_tcp_stream(ip_address: &str, port: u16) -> Result<TcpStream, String> {
    let reply: Result<TcpStream, String>;

    let address_result: Result<SocketAddr, String> = parse_network_address(ip_address, port);

    if address_result.is_ok() {
        let connection_result: io::Result<TcpStream> = TcpStream::connect(address_result.unwrap());
        if connection_result.is_ok() {
            reply = Ok(connection_result.unwrap());
        } else {
            reply = Err(connection_result.unwrap_err().description().to_owned());
        }
    } else {
        reply = Err(address_result.unwrap_err());
    }

    return reply;
}
