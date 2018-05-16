

use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;

//	===============================================================================================

#[test]
fn test_parse_network_address ()
{
    let result_1 : Option< SocketAddr > = parse_network_address ( "127.0.0.1", 
                                                                  502 );
    assert! ( result_1.is_some () );

    let result_2 : Option< SocketAddr > = parse_network_address ( "127.0.0.1:504",
                                                                  502 );
    assert! ( result_2.is_some () );

    let socket_1 : SocketAddr = result_2.unwrap ();
    assert! ( socket_1.is_ipv4 () );
    assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    assert_eq! ( socket_1.port (), 504 );

    let result_3 : Option< SocketAddr > = parse_network_address ( "127.0.300.1",
                                                                  502 );
    assert! ( result_3.is_none () );

    let result_4 : Option< SocketAddr > = parse_network_address ( "::1", 
                                                                  502 );
    assert! ( result_4.is_some () );

    let result_5 : Option< SocketAddr > = parse_network_address ( "[::1]:504", 
                                                                  502 );
    assert! ( result_5.is_some () );

    let socket_2 : SocketAddr = result_5.unwrap ();
    assert! ( socket_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    assert_eq! ( socket_2.port (), 504 );

    let result_6 : Option< SocketAddr > = parse_network_address ( "::111111", 
                                                                  502 );
    assert! ( result_6.is_none () );

    let result_7 : Option< SocketAddr > = parse_network_address ( "127.0.0.1", 
                                                                  0 );
    assert! ( result_7.is_none () );

    let result_8 : Option< SocketAddr > = parse_network_address ( "", 
                                                                  502 );
    assert! ( result_8.is_none () );
}

pub fn parse_network_address ( address_string : &str, default_port : u16 ) -> Option< SocketAddr >
{
    let reply : Option< SocketAddr >;

    if address_string.is_empty () || default_port == 0x0000
    {
        reply = None;
    }
    else
    {
        if let Some( socket ) = parse_socket_address ( address_string )
        {
            reply = Some( socket );
        }
        else
        {
            if let Some( ip ) = parse_ip_address ( address_string )
            {
                reply = Some ( SocketAddr::new ( ip, default_port ) );
            }
            else
            {
                reply = None;
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_parse_ip_address ()
{
    let result_1 : Option< IpAddr > = parse_ip_address ( "127.0.0.1" );
    assert! ( result_1.is_some () );

    let ip_1 : IpAddr = result_1.unwrap ();
    assert! ( ip_1.is_ipv4 () );
    assert_eq! ( format!("{}", ip_1 ), "127.0.0.1" );

    let result_2 : Option< IpAddr > = parse_ip_address ( "127.0.0.1111" );
    assert! ( result_2.is_none () );

    let result_3 : Option< IpAddr > = parse_ip_address ( "::1" );
    assert! ( result_3.is_some () );

    let ip_2 : IpAddr = result_3.unwrap ();
    assert! ( ip_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", ip_2 ), "::1" );

    let result_4 : Option< IpAddr > = parse_ip_address ( "::111111" );
    assert! ( result_4.is_none () );

    let result_5 : Option< IpAddr > = parse_ip_address ( "" );
    assert! ( result_5.is_none () );
}

fn parse_ip_address ( address_string : &str ) -> Option< IpAddr >
{
    let reply : Option< IpAddr >;

    if address_string.is_empty ()
    {
        reply = None;
    }
    else
    {
        let result : Result< IpAddr, AddrParseError > = IpAddr::from_str ( address_string );

        if result.is_ok ()
        {
            reply = Some( result.unwrap () );
        }
        else
        {
            reply = None;
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_parse_socket_address ()
{
    let result_1 : Option< SocketAddr > = parse_socket_address ( "127.0.0.1:502" );
    assert! ( result_1.is_some () );
    
    let socket_1 : SocketAddr = result_1.unwrap ();
    assert! ( socket_1.is_ipv4 () );
    assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    assert_eq! ( socket_1.port (), 502 );

    let result_2 : Option< SocketAddr > = parse_socket_address ( "127.0.0.1111:502" );
    assert! ( result_2.is_none () );

    let result_3 : Option< SocketAddr > = parse_socket_address ( "127.0.0.1" );
    assert! ( result_3.is_none () );

    let result_4 : Option< SocketAddr > = parse_socket_address ( "[::1]:502" );
    assert! ( result_4.is_some () );

    let socket_2 : SocketAddr = result_4.unwrap ();
    assert! ( socket_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    assert_eq! ( socket_2.port (), 502 );

    let result_5 : Option< SocketAddr > = parse_socket_address ( "[::111111]:502" );
    assert! ( result_5.is_none () );

    let result_6 : Option< SocketAddr > = parse_socket_address ( "[::1]" );
    assert! ( result_6.is_none () );

    let result_7 : Option< SocketAddr > = parse_socket_address ( "" );
    assert! ( result_7.is_none () );
}

fn parse_socket_address ( address_string : &str ) -> Option< SocketAddr >
{
    let reply : Option< SocketAddr >;

    if address_string.is_empty ()
    {
        reply = None;
    }
    else
    {
        let result : Result< SocketAddr, AddrParseError > = SocketAddr::from_str ( address_string );

        if result.is_ok ()
        {
            reply = Some( result.unwrap () );
        }
        else
        {
            reply = None;
        }
    }

    return reply;
}
