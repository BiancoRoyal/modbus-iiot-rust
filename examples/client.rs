

extern crate modbus_iiot;


use modbus_iiot::tcp::master::*;
use modbus_iiot::core::ethernet::EthernetMaster;


fn main()
{
    println!("Hello, Modbus world!");
    let mut client = TcpClient::new("192.168.1.1");
    client.connect();

    // read register from address 266 (WAGO controllers)
    let response = client.read_holding_registers(266, 1);
    println!("Response: {:?}", response);

    // read 10 words starting at address 256 (WAGO controllers)
    let response = client.read_holding_registers(256, 10);
    println!("Response: {:?}", response);
    client.disconnect();
}
