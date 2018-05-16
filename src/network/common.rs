

use std::net::TcpStream;
use network::socket::parse_network_address;

//	===============================================================================================

pub fn create_tcp_stream ( ip_address : &str, port : u16 ) -> Option< TcpStream >
{	
	let reply : Option< TcpStream >;

	if let Some( socket_address ) = parse_network_address ( ip_address, 
															port )
	{
		if let Ok( stream ) = TcpStream::connect ( socket_address )
		{
			reply = Some( stream );
		}
		else
		{
			reply = None;
		}
	}
	else
	{
		reply = None;
	}

	return reply;		
}
