use std::error::Error;
use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;

//	===============================================================================================

#[test]
fn test_parse_network_address() {
    let result_1: Result<SocketAddr, String> = parse_network_address("127.0.0.1", 502);
    assert!(result_1.is_ok());

    let result_2: Result<SocketAddr, String> = parse_network_address("127.0.0.1:504", 502);
    assert!(result_2.is_ok());

    let socket_1: SocketAddr = result_2.unwrap();
    assert!(socket_1.is_ipv4());
    assert_eq!(format!("{}", socket_1.ip()), "127.0.0.1");
    assert_eq!(socket_1.port(), 504);

    let result_3: Result<SocketAddr, String> = parse_network_address("127.0.300.1", 502);
    assert!(result_3.is_err());

    let result_4: Result<SocketAddr, String> = parse_network_address("::1", 502);
    assert!(result_4.is_ok());

    let result_5: Result<SocketAddr, String> = parse_network_address("[::1]:504", 502);
    assert!(result_5.is_ok());

    let socket_2: SocketAddr = result_5.unwrap();
    assert!(socket_2.is_ipv6());
    assert_eq!(format!("{}", socket_2.ip()), "::1");
    assert_eq!(socket_2.port(), 504);

    let result_6: Result<SocketAddr, String> = parse_network_address("::111111", 502);
    assert!(result_6.is_err());

    let result_7: Result<SocketAddr, String> = parse_network_address("127.0.0.1", 0);
    assert!(result_7.is_err());

    let result_8: Result<SocketAddr, String> = parse_network_address("", 502);
    assert!(result_8.is_err());
}

pub fn parse_network_address(
    address_string: &str,
    default_port: u16,
) -> Result<SocketAddr, String> {
    let reply: Result<SocketAddr, String>;

    if address_string.is_empty() || default_port == 0x0000 {
        reply = Err("address is empty or port is 0.".to_string());
    } else {
        if let Ok(socket) = parse_socket_address(address_string) {
            reply = Ok(socket);
        } else {
            let ip: Result<IpAddr, String> = parse_ip_address(address_string);

            if ip.is_ok() {
                reply = Ok(SocketAddr::new(ip.unwrap(), default_port));
            } else {
                reply = Err(ip.unwrap_err());
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_parse_ip_address() {
    let result_1: Result<IpAddr, String> = parse_ip_address("127.0.0.1");
    assert!(result_1.is_ok());

    let ip_1: IpAddr = result_1.unwrap();
    assert!(ip_1.is_ipv4());
    assert_eq!(format!("{}", ip_1), "127.0.0.1");

    let result_2: Result<IpAddr, String> = parse_ip_address("127.0.0.1111");
    assert!(result_2.is_err());

    let result_3: Result<IpAddr, String> = parse_ip_address("::1");
    assert!(result_3.is_ok());

    let ip_2: IpAddr = result_3.unwrap();
    assert!(ip_2.is_ipv6());
    assert_eq!(format!("{}", ip_2), "::1");

    let result_4: Result<IpAddr, String> = parse_ip_address("::111111");
    assert!(result_4.is_err());

    let result_5: Result<IpAddr, String> = parse_ip_address("");
    assert!(result_5.is_err());
}

fn parse_ip_address(address_string: &str) -> Result<IpAddr, String> {
    let reply: Result<IpAddr, String>;

    if address_string.is_empty() {
        reply = Err("address is empty.".to_string());
    } else {
        let result: Result<IpAddr, AddrParseError> = IpAddr::from_str(address_string);

        if result.is_ok() {
            reply = Ok(result.unwrap());
        } else {
            reply = Err(result.unwrap_err().description().to_owned());
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_parse_socket_address() {
    let result_1: Result<SocketAddr, String> = parse_socket_address("127.0.0.1:502");
    assert!(result_1.is_ok());

    let socket_1: SocketAddr = result_1.unwrap();
    assert!(socket_1.is_ipv4());
    assert_eq!(format!("{}", socket_1.ip()), "127.0.0.1");
    assert_eq!(socket_1.port(), 502);

    let result_2: Result<SocketAddr, String> = parse_socket_address("127.0.0.1111:502");
    assert!(result_2.is_err());

    let result_3: Result<SocketAddr, String> = parse_socket_address("127.0.0.1");
    assert!(result_3.is_err());

    let result_4: Result<SocketAddr, String> = parse_socket_address("[::1]:502");
    assert!(result_4.is_ok());

    let socket_2: SocketAddr = result_4.unwrap();
    assert!(socket_2.is_ipv6());
    assert_eq!(format!("{}", socket_2.ip()), "::1");
    assert_eq!(socket_2.port(), 502);

    let result_5: Result<SocketAddr, String> = parse_socket_address("[::111111]:502");
    assert!(result_5.is_err());

    let result_6: Result<SocketAddr, String> = parse_socket_address("[::1]");
    assert!(result_6.is_err());

    let result_7: Result<SocketAddr, String> = parse_socket_address("");
    assert!(result_7.is_err());
}

fn parse_socket_address(address_string: &str) -> Result<SocketAddr, String> {
    let reply: Result<SocketAddr, String>;

    if address_string.is_empty() {
        reply = Err("address is empty.".to_string());
    } else {
        let result: Result<SocketAddr, AddrParseError> = SocketAddr::from_str(address_string);

        if result.is_ok() {
            reply = Ok(result.unwrap());
        } else {
            reply = Err(result.unwrap_err().description().to_owned());
        }
    }

    return reply;
}
