

use std::net::{TcpStream, Shutdown};
use std::result::Result;
use std::time::Duration;
use network::common::create_tcp_stream;
use core::consts::*;
use core::ethernet::*;
use core::modbustelegram::*;
use core::modbusreturn::*;
use core::methods::*;
use core::timehandling::*;
use tcp::masteraccess::*;
use tcp::streamtelegram::*;

//	===============================================================================================

pub struct TcpClient
{	
	address : String,	
	last_transaction_id : u16,	
    port : u16,    
    stream : Option < TcpStream >,     
	unit_identifier : u8    
}

impl TcpClient
{	
	pub fn new ( address : &str ) -> TcpClient
	{		
		return Self::new_with_port ( address, 
									 MODBUS_TCP_PORT );		
	}
	
	pub fn new_with_port ( address : &str, port : u16 ) -> TcpClient
	{		
		return Self::new_with_port_and_unitid ( address, 
												port, 
												MODBUS_DEFAULT_UNIT_IDENTIFIER );		
	}
	
	pub fn new_with_port_and_unitid ( address : &str, port : u16, unit_id : u8 ) -> TcpClient
	{
		return TcpClient
		{
			address : address.to_string (),
			last_transaction_id : MODBUS_TRANSACTION_ID_INITIALIZER,
			port : port,
			stream : None,
			unit_identifier : unit_id
		};
	}

	pub fn connect ( &mut self ) -> Result< bool, String >
	{
		let reply : Result< bool, String >;

		let connection_result : Result< TcpStream, String > = create_tcp_stream ( &self.address, 
																				  self.port );

		if connection_result.is_ok ()
		{
			let connection : TcpStream = connection_result.unwrap ();

			let timeout : Duration = Duration::from_millis ( 500 );

			let _ = connection.set_read_timeout ( Some( timeout ) );
			let _ = connection.set_write_timeout ( Some( timeout ) );
			let _ = connection.set_nodelay ( true );

			self.stream = Some( connection );

			reply = Ok( true );
		}
		else
		{
			reply = Err( connection_result.unwrap_err () );
		}

		return reply;
	}

	pub fn disconnect ( &mut self ) -> bool
	{	
		let mut reply : bool = false;	

		if self.stream.is_some ()
		{			
			if let Some( connection ) = self.stream.take ()
			{
				if let Ok( _ ) = connection.shutdown ( Shutdown:: Both )
				{
					reply = true;
				}				
			}
		}

		return reply;
	}

	fn process_telegram ( &mut self, request : &Option< ModbusTelegram > ) -> Option< ModbusTelegram >
	{
		let mut reply : Option< ModbusTelegram > = None;

		if let Some( mut stream ) = self.stream.take ()
		{			
			reply = process_modbus_telegram ( &mut stream, 
											  &request );

			self.stream = Some ( stream );
			self.update_last_transaction_id ();
		}

		return reply;
	}

	fn update_last_transaction_id ( &mut self )
	{
		self.last_transaction_id = count_up_last_transaction_id ( self.last_transaction_id );
	}
}

//	===============================================================================================

#[test]
fn test_count_up_last_transaction_id ()
{
	let test_data_1 : u16 = 0x0001;
	let result_data_1 : u16 = count_up_last_transaction_id ( test_data_1 );
	assert_eq! ( result_data_1, 0x0002 );

	let test_data_2 : u16 = 0xFFFF;
	let result_data_2 : u16 = count_up_last_transaction_id ( test_data_2 );
	assert_eq! ( result_data_2, 0x0001 );
}

fn count_up_last_transaction_id ( last_transaction_id : u16 ) -> u16
{
	let reply : u16;

	if last_transaction_id == 0xFFFF
	{
		reply = MODBUS_TRANSACTION_ID_INITIALIZER;
	}
	else
	{
		reply = last_transaction_id + 1;
	}

	return reply;
}

//	===============================================================================================

impl EthernetMaster for TcpClient
{
	fn read_coils ( &mut self, starting_address : u16, quantity_of_coils : u16 ) -> ModbusReturnCoils
	{
		let reply : ModbusReturnCoils;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result < ModbusTelegram, String > = create_request_read_coils ( self.last_transaction_id, 
																							   self.unit_identifier, 
																							   starting_address, 
																							   quantity_of_coils );
	
		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (), 
										  &response )
				{
					let response_data : Vec< bool > = prepare_response_read_coils ( &response.get_payload ().unwrap (),
																					quantity_of_coils );

					reply = process_response_of_coils ( response_data,
														&start_time );
				}
				else
				{
					reply = ModbusReturnCoils::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}				
			}
			else
			{
				reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn read_discrete_inputs ( &mut self, starting_address : u16, quantity_of_inputs : u16 ) -> ModbusReturnCoils
	{
		let reply : ModbusReturnCoils;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_read_discrete_inputs ( self.last_transaction_id, 
																					 		   			self.unit_identifier, 
																					 		   			starting_address, 
																					 		   			quantity_of_inputs );
	
		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request ) 
			{					
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< bool > = prepare_response_read_discrete_inputs ( &response.get_payload ().unwrap (),
																							  quantity_of_inputs );

					reply = process_response_of_coils ( response_data,
														&start_time );
				}
				else
				{
					reply = ModbusReturnCoils::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn read_holding_registers ( &mut self, starting_address : u16, quantity_of_registers : u16 ) -> ModbusReturnRegisters
	{
		let reply : ModbusReturnRegisters;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_read_holding_registers ( self.last_transaction_id, 
																										  self.unit_identifier,
																										  starting_address,
																										  quantity_of_registers );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{				
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< u16 > = prepare_response_read_holding_registers ( &response.get_payload ().unwrap () );

					reply = process_response_of_registers ( response_data,
															&start_time );
				}
				else
				{
					reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( &request_telegram.err ( ).unwrap ( ) ) );
		}

		return reply;		
	}

	fn read_input_registers ( &mut self, starting_address : u16, quantity_of_input_registers : u16 ) -> ModbusReturnRegisters
	{
		let reply : ModbusReturnRegisters;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_read_input_registers ( self.last_transaction_id, 
																							   			self.unit_identifier, 
																							   			starting_address, 
																							   			quantity_of_input_registers );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< u16 > = prepare_response_read_input_registers ( &response.get_payload ().unwrap () );

					reply = process_response_of_registers ( response_data,
															&start_time );
				}
				else
				{
					reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn write_single_coil ( &mut self, output_address : u16, output_value : u16 ) -> ModbusReturnCoils
	{
		let reply : ModbusReturnCoils;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_write_single_coil ( self.last_transaction_id, 
																						    		 self.unit_identifier, 
																						    		 output_address, 
																						    		 output_value );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< bool > = prepare_response_write_single_coil ( &response.get_payload ().unwrap () );

					reply = process_response_of_coils ( response_data,
														&start_time );
				}
				else
				{
					reply = ModbusReturnCoils::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}			
		}
		else
		{
			reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn write_single_register ( &mut self, register_address : u16, register_value : u16 ) -> ModbusReturnRegisters
	{
		let reply : ModbusReturnRegisters;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_write_single_register ( self.last_transaction_id, 
																						    			 self.unit_identifier, 
																						    			 register_address, 
																						    			 register_value );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );
			
			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< u16 > = prepare_response_write_single_register ( &response.get_payload ().unwrap () );

					reply = process_response_of_registers ( response_data,
															&start_time );
				}
				else
				{
					reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}			
		}
		else
		{
			reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn write_multiple_coils ( &mut self, starting_address : u16, quantity_of_outputs : u16, outputs_value : Vec< u8 > ) -> ModbusReturnRegisters
	{
		let reply: ModbusReturnRegisters;

		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_write_multiple_coils ( self.last_transaction_id, 
																						       			self.unit_identifier, 
																						       			starting_address, 
																						       			quantity_of_outputs,
																							   			outputs_value );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< u16 > = prepare_response_write_multiple_coils ( &response.get_payload ().unwrap () );

					reply = process_response_of_registers ( response_data,
															&start_time );
				}
				else
				{
					reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}

	fn write_multiple_registers ( &mut self, starting_address : u16, register_values : Vec< u16 > ) -> ModbusReturnRegisters
	{
		let reply: ModbusReturnRegisters;
		
		let start_time : Timestamp = Timestamp::new ();
		let request_telegram : Result< ModbusTelegram, String > = create_request_write_multiple_registers ( self.last_transaction_id, 
																								   			self.unit_identifier, 
																								   			starting_address, 
																								   			register_values );

		if request_telegram.is_ok ()
		{
			let request : Option< ModbusTelegram > = Some( request_telegram.unwrap () );

			if let Some( response ) = self.process_telegram ( &request )
			{
				if verify_function_code ( &request.unwrap (),
										  &response )
				{
					let response_data : Vec< u16 > = prepare_response_write_multiple_registers ( &response.get_payload ().unwrap () );

					reply = process_response_of_registers ( response_data,
															&start_time );
				}
				else
				{
					reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( response.get_function_code ().unwrap (), 1 ) );
				}					
			}
			else
			{
				reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
			}
		}
		else
		{
			reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( &request_telegram.err ().unwrap () ) );
		}

		return reply;
	}
}

//	===============================================================================================

#[test]
fn test_response_of_coils ()
{
	let timestamp : Timestamp = Timestamp::new ();

	let test_data_1 : Vec< bool > = vec![ false, false, true, false, true, true, false, true ];
	let result_1 : ModbusReturnCoils = process_response_of_coils ( test_data_1,
																   &timestamp );
	assert! ( result_1.is_good () );

	let test_data_2 : Vec< bool > = vec![];
	let result_2 : ModbusReturnCoils = process_response_of_coils ( test_data_2,
																   &timestamp );
	assert! ( result_2.is_bad () );
}

fn process_response_of_coils ( response_data : Vec< bool >, start_time : &Timestamp ) -> ModbusReturnCoils
{
	let reply : ModbusReturnCoils;

	if response_data.len () > 0
	{
		reply = ModbusReturnCoils::Good( ReturnGood::new ( response_data, 
														   start_time.elapsed_milliseconds () ) );
	}
	else
	{
		reply = ModbusReturnCoils::Bad( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
	}

	return reply;
}

//	===============================================================================================

#[test]
fn test_process_response_of_registers ()
{
	let timestamp : Timestamp = Timestamp::new ();

	let test_data_1 : Vec< u16 > = vec![ 0x000A, 0xFFFF, 0x00A8, 0xFF00 ];
	let result_1 : ModbusReturnRegisters = process_response_of_registers ( test_data_1,
																   		   &timestamp );
	assert! ( result_1.is_good () );

	let test_data_2 : Vec< u16 > = vec![];
	let result_2 : ModbusReturnRegisters = process_response_of_registers ( test_data_2,
																   		   &timestamp );
	assert! ( result_2.is_bad () );
}

fn process_response_of_registers ( response_data : Vec< u16 >, start_time : &Timestamp ) -> ModbusReturnRegisters
{
	let reply : ModbusReturnRegisters;

	if response_data.len () > 0
	{
		reply = ModbusReturnRegisters::Good( ReturnGood::new ( response_data, 
															   start_time.elapsed_milliseconds () ) );
	}
	else
	{
		reply = ModbusReturnRegisters::Bad( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
	}

	return reply;
}

//	===============================================================================================

impl MasterAccess for TcpClient
{
	fn read_coils ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >
	{
		let reply : Vec< CoilValue >;

		let response : ModbusReturnCoils = EthernetMaster::read_coils ( self, 
																		address, 
																		quantity );

		if response.is_good ()
		{
			reply = transform_modbus_return_coils ( response );
		}
		else
		{
			reply = vec![];
		}

		return reply;
	}

	fn read_discrete_inputs ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >
	{
		let reply : Vec< CoilValue >;

		let response : ModbusReturnCoils = EthernetMaster::read_discrete_inputs ( self, 
																			   	  address, 
																			      quantity );

		if response.is_good ()
		{
			reply = transform_modbus_return_coils ( response );
		}
		else
		{
			reply = vec![];
		}

		return reply;
	}

	fn read_holding_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >
	{
		let reply : Vec< u16 >;

		let response : ModbusReturnRegisters = EthernetMaster::read_holding_registers ( self,
																						address,
																						quantity );

		if response.is_good ()
		{
			reply = transform_modbus_return_registers ( response );
		}
		else
		{
			reply = vec![];
		}

		return reply;
	}

	fn read_input_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >
	{
		let reply : Vec< u16 >;

		let response : ModbusReturnRegisters = EthernetMaster::read_input_registers ( self,
																					  address,
																					  quantity );

		if response.is_good ()
		{
			reply = transform_modbus_return_registers ( response );
		}
		else
		{
			reply = vec![];
		}

		return reply;
	}

	fn write_single_coil ( &mut self, address : u16, value : CoilValue ) -> bool
	{
		let mut reply : bool = false;

		let response : ModbusReturnCoils = EthernetMaster::write_single_coil ( self, 
																			   address, 
																			   convert_for_write_single_coil ( &value ) );

		if response.is_good ()
		{
			reply = true;
		}

		return reply;
	}

	fn write_single_register ( &mut self, address : u16, value : u16 ) -> bool
	{
		let response : ModbusReturnRegisters = EthernetMaster::write_single_register ( self,
																					   address,
																					   value );

		return response.is_good ();
	}

	fn write_multiple_coils ( &mut self, address : u16, coils : Vec< CoilValue > ) -> bool
	{
		let mut reply : bool = false;

		if coils.len () > 0
		{
			let values : Vec< u8 > = transform_coils_to_bytearray ( &coils );
			let response : ModbusReturnRegisters = EthernetMaster::write_multiple_coils ( self, 
																						  address, 
																			   			  coils.len () as u16,
																						  values );

			reply = response.is_good ();
		}

		return reply;
	}	

	fn write_multiple_registers ( &mut self, address : u16, values : Vec< u16 > ) -> bool
	{
		let response : ModbusReturnRegisters = EthernetMaster::write_multiple_registers ( self,
																						  address,
																						  values );

		return response.is_good ();
	}
}

//	===============================================================================================

fn transform_modbus_return_coils ( returned_coils : ModbusReturnCoils ) -> Vec< CoilValue >
{
	let mut reply : Vec< CoilValue > = vec![];

	if returned_coils.is_good ()
	{
		let values : Vec< bool > = returned_coils.unwrap_good ().get_data ();

		for coil in values
		{
			reply.push ( CoilValue::set ( coil ) );
		}
	}

	return reply;
}

//	===============================================================================================

fn transform_modbus_return_registers ( returned_registers : ModbusReturnRegisters ) -> Vec< u16 >
{
	let mut reply : Vec< u16 > = vec![];

	if returned_registers.is_good ()
	{
		reply = returned_registers.unwrap_good ().get_data ();
	}

	return reply;
}
