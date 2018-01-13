

use std::result::Result;
use std::net::{Ipv4Addr, AddrParseError};
use std::str::FromStr;


pub fn to_socket_address_v4 ( host : &str ) -> Option < Ipv4Addr >
{
	let mut l_return : Option<Ipv4Addr> = None;
	
	let l_ip : Result<Ipv4Addr, AddrParseError> = Ipv4Addr::from_str ( &host );

    if l_ip.is_ok ( )
    {
        l_return = Some ( l_ip.unwrap ( ) );        
    }

	return l_return;	
}
