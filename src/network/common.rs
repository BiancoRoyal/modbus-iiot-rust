

use std::io::Result;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

use super::socket::to_socket_address_v4;


pub fn create_tcp_stream_v4 ( host : &str, port : u16 ) -> Option < TcpStream >
{	
	let mut l_return : Option<TcpStream> = None;

	let l_address : Option<Ipv4Addr> = to_socket_address_v4 ( &host );
	
	if l_address.is_some ( )
	{
		let ip : Ipv4Addr = l_address.unwrap ( );
		
		let socket : SocketAddrV4 = SocketAddrV4::new ( ip, port );
		
		let l_connection_result : Result<TcpStream> = TcpStream::connect ( socket );

		if l_connection_result.is_ok()
		{
			l_return = Some ( l_connection_result.unwrap ( ) );
		}
	}		

	return l_return;		
}
