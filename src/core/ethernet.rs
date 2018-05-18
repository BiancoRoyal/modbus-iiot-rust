

use core::modbusreturn::{ModbusReturnCoils, ModbusReturnRegisters};

//	===============================================================================================

pub trait EthernetMaster
{	
	///	Function Code 1 (0x01)
	fn read_coils ( &mut self, starting_address : u16, quantity_of_coils : u16 ) -> ModbusReturnCoils;

	///	Function Code 2 (0x02)
    fn read_discrete_inputs ( &mut self, starting_address : u16, quantity_of_inputs : u16 ) -> ModbusReturnCoils;

	///	Function Code 3 (0x03)
    fn read_holding_registers ( &mut self, starting_address : u16, quantity_of_registers : u16 ) -> ModbusReturnRegisters;
   
    ///	Function Code 4 (0x04)
    fn read_input_registers ( &mut self, starting_address : u16, quantity_of_input_registers : u16 ) -> ModbusReturnRegisters;
    
	/// Function Code 5 (0x05)
    fn write_single_coil ( &mut self, output_address : u16, output_value : u16 ) -> ModbusReturnCoils;
  
    ///	Function Code 6 (0x06)
    fn write_single_register ( &mut self, register_address : u16, register_value : u16 ) -> ModbusReturnRegisters;
    
    ///	Function Code 15 (0x0F)
    fn write_multiple_coils ( &mut self, starting_address : u16, quantity_of_outputs : u16, outputs_value : Vec< u8 > ) -> ModbusReturnRegisters;
    
    ///	Function Code 16 (0x10)
    fn write_multiple_registers ( &mut self, starting_address : u16, register_values : Vec< u16 > ) -> ModbusReturnRegisters;    
}
