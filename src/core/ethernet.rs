

use core::modbusreturn::{ModbusReturnCoils, ModbusReturnRegisters};

//	===============================================================================================
/// EthernetMaster - Modbus functions of the master (client)
/// 
/// 
/// ```
/// extern crate modbus_iiot;
/// 
/// use modbus_iiot::tcp::master::*;
/// use modbus_iiot::core::ethernet::EthernetMaster;
/// 
/// fn main() {
///     let mut client = TcpClient::new_with_port("192.168.1.1", 10502);
///     if client.connect() {
///         println!("connected");
///         // your code to call Modbus functions
///     }
///     else {
///         println!("not connected");
///     }
/// }
/// 
/// ```
pub trait EthernetMaster
{	
	///	Function Code 1 (0x01)
    /// ```
    /// let response = client.read_coils(0, 1);
    /// println!("Response CO: {:?}", response);
    /// ```
	fn read_coils ( &mut self, starting_address : u16, quantity_of_coils : u16 ) -> ModbusReturnCoils;

	///	Function Code 2 (0x02)
    /// ```
    /// let response = client.read_discrete_inputs(0, 1);
    /// println!("Response of read_discrete_inputs: {:?}", response);
    /// ```
    fn read_discrete_inputs ( &mut self, starting_address : u16, quantity_of_inputs : u16 ) -> ModbusReturnCoils;

	///	Function Code 3 (0x03)
    /// ```
    /// let response = client.read_holding_registers(0, 1);
    /// println!("Response of read_holding_registers: {:?}", response);
    /// ```
    fn read_holding_registers ( &mut self, starting_address : u16, quantity_of_registers : u16 ) -> ModbusReturnRegisters;
   
    ///	Function Code 4 (0x04)
    /// ```
    /// let response = client.read_input_registers(0, 1);
    /// println!("Response of read_input_registers: {:?}", response);
    /// ```
    fn read_input_registers ( &mut self, starting_address : u16, quantity_of_input_registers : u16 ) -> ModbusReturnRegisters;
    
	/// Function Code 5 (0x05)
    /// ```
    /// let response = client.write_single_coil(0, 1);
    /// println!("Response of write_single_coil: {:?}", response);
    /// ```
    fn write_single_coil ( &mut self, output_address : u16, output_value : u16 ) -> ModbusReturnCoils;
  
    ///	Function Code 6 (0x06)
    /// ```
    /// let response = client.write_single_register(0, 1);
    /// println!("Response of write_single_register: {:?}", response);
    /// ```
    fn write_single_register ( &mut self, register_address : u16, register_value : u16 ) -> ModbusReturnRegisters;
    
    ///	Function Code 15 (0x0F)
    /// ```
    /// let response = client.write_multiple_coils(0, 3, vec![ 1, 0, 1 ]);
    /// println!("Response of write_multiple_coils: {:?}", response);
    /// ```
    fn write_multiple_coils ( &mut self, starting_address : u16, quantity_of_outputs : u16, outputs_value : Vec< u8 > ) -> ModbusReturnRegisters;
    
    ///	Function Code 16 (0x10)
    /// ```
    /// let response = client.write_multiple_registers(0, 3, vec![ 1000, 2000, 3000 ]);
    /// println!("Response of write_multiple_registers: {:?}", response);
    /// ```
    fn write_multiple_registers ( &mut self, starting_address : u16, register_values : Vec< u16 > ) -> ModbusReturnRegisters;    
}
