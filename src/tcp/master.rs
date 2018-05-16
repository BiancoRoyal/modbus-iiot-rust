

use std::net::{TcpStream, Shutdown};
use std::result::Result;
use std::time::{Duration, SystemTime};
use network::common::create_tcp_stream;
use core::ethernet::*;
use core::modbustelegram::*;
use core::modbusreturn::{ModbusReturnCoils, ModbusReturnRegisters, ReturnBad, ReturnGood};
use core::methods::*;
use core::timehandling::*;
use tcp::streamtelegram::*;


const MODBUS_DEFAULT_UNIT_IDENTIFIER : u8 = 0x01;
const MODBUS_TCP_PORT : u16 = 502;
const MODBUS_TRANSACTION_ID_INITIALIZER : u16 = 0x0001;


pub struct TcpClient
{	
	//	IPv4-Adresse, IPv6-Addresse oder Host-Name im Netzwerk
	address : String,	
	//	Letzte Transaktionsnummer im Protokollkopf
	last_transaction_id : u16,	
	//	Tcp-Port
    port : u16,    
	//	TcpClient-Verbindung
    stream : Option<TcpStream>,     
	//	UnitId, SlaveId ( Gerätekennung, bei Tcp für gewöhnlich 1 )
    unit_identifier : u8    
}


impl TcpClient
{	
	pub fn new ( address : &str ) -> TcpClient
	{		
		return Self::new_with_port ( address, MODBUS_TCP_PORT );		
	}
	
	pub fn new_with_port ( address : &str, port : u16 ) -> TcpClient
	{		
		return Self::new_with_port_and_unitid ( address, port, MODBUS_DEFAULT_UNIT_IDENTIFIER );		
	}
	
	pub fn new_with_port_and_unitid ( address : &str, port : u16, unit_id : u8 ) -> TcpClient
	{
		return TcpClient
			{
				address : address.to_string ( ),
				last_transaction_id : MODBUS_TRANSACTION_ID_INITIALIZER,
				port : port,
				stream : None,
				unit_identifier : unit_id
			};
	}

	pub fn connect ( &mut self ) -> bool
	{
		let reply : bool;

		if let Some( connection ) = create_tcp_stream ( &self.address, 
														self.port )
		{
			let timeout : Duration = Duration::from_millis ( 500 );

			let _ = connection.set_read_timeout ( Some( timeout ) );
			let _ = connection.set_write_timeout ( Some( timeout ) );
			let _ = connection.set_nodelay ( true );

			self.stream = Some( connection );

			reply = true;
		}
		else
		{
			reply = false;
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

	pub fn update_last_transaction_id ( &mut self )
	{
		self.last_transaction_id = count_up_last_transaction_id ( self.last_transaction_id );
	}
}

//	===============================================================================================

#[test]
fn test_count_up_last_transaction_id ( )
{
	let l_test_1_data : u16 = 0x0001;

	let l_result_1_data : u16 = count_up_last_transaction_id ( l_test_1_data );

	assert_eq! ( l_result_1_data, 0x0002 );


	let l_test_2_data : u16 = 0xFFFF;

	let l_result_2_data : u16 = count_up_last_transaction_id ( l_test_2_data );

	assert_eq! ( l_result_2_data, 0x0001 );
}

fn count_up_last_transaction_id ( last_transaction_id : u16 ) -> u16
{
	let l_return : u16;

	if last_transaction_id == 0xFFFF
	{
		l_return = MODBUS_TRANSACTION_ID_INITIALIZER;
	}
	else
	{
		l_return = last_transaction_id + 1;
	}

	return l_return;
}

//	===============================================================================================

impl EthernetMaster for TcpClient
{

	fn read_coils ( &mut self, starting_address : u16, quantity_of_coils : u16 ) -> ModbusReturnCoils
	{
		let l_return: ModbusReturnCoils;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_read_coils ( self.last_transaction_id, 
																					 self.unit_identifier, 
																					 starting_address, 
																					 quantity_of_coils );
	
		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<bool> = prepare_response_read_coils ( &l_response.get_payload ( ).unwrap ( ),
																		quantity_of_coils );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnCoils::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn read_discrete_inputs ( &mut self, starting_address : u16, quantity_of_inputs : u16 ) -> ModbusReturnCoils
	{
		let l_return: ModbusReturnCoils;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_read_discrete_inputs ( self.last_transaction_id, 
																					 		   self.unit_identifier, 
																					 		   starting_address, 
																					 		   quantity_of_inputs );
	
		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<bool> = prepare_response_read_discrete_inputs ( &l_response.get_payload ( ).unwrap ( ),
																				  quantity_of_inputs );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnCoils::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn read_holding_registers ( &mut self, starting_address : u16, quantity_of_registers : u16 ) -> ModbusReturnRegisters
	{
		let l_return: ModbusReturnRegisters;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_read_holding_registers ( self.last_transaction_id, 
																								 self.unit_identifier, 
																								 starting_address, 
																								 quantity_of_registers );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<u16> = prepare_response_read_holding_registers ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnRegisters::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }

					}
					else
					{
						l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;		
	}

	fn read_input_registers ( &mut self, starting_address : u16, quantity_of_input_registers : u16 ) -> ModbusReturnRegisters
	{
		let l_return : ModbusReturnRegisters;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_read_input_registers ( self.last_transaction_id, 
																							   self.unit_identifier, 
																							   starting_address, 
																							   quantity_of_input_registers );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<u16> = prepare_response_read_input_registers ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnRegisters::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn write_single_coil ( &mut self, output_address : u16, output_value : u16 ) -> ModbusReturnCoils
	{
		let l_return: ModbusReturnCoils;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_write_single_coil ( self.last_transaction_id, 
																						    self.unit_identifier, 
																						    output_address, 
																						    output_value );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<bool> = prepare_response_write_single_coil ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnCoils::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnCoils::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn write_single_register ( &mut self, register_address : u16, register_value : u16 ) -> ModbusReturnRegisters
	{
		let l_return: ModbusReturnRegisters;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_write_single_register ( self.last_transaction_id, 
																						    	self.unit_identifier, 
																						    	register_address, 
																						    	register_value );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<u16> = prepare_response_write_single_register ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnRegisters::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn write_multiple_coils ( &mut self, starting_address : u16, quantity_of_outputs : u16, outputs_value : Vec<u8> ) -> ModbusReturnRegisters
	{
		let l_return: ModbusReturnRegisters;

		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_write_multiple_coils ( self.last_transaction_id, 
																						       self.unit_identifier, 
																						       starting_address, 
																						       quantity_of_outputs,
																							   outputs_value );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream =	l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<u16> = prepare_response_write_multiple_coils ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnRegisters::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

	fn write_multiple_registers ( &mut self, starting_address : u16, register_values : Vec<u16> ) -> ModbusReturnRegisters
	{
		let l_return: ModbusReturnRegisters;
		
		//	PERFORMANCE-Info
		let l_start_time : SystemTime =	SystemTime::now ( );

		let l_request : Result<ModbusTelegram, String> = create_request_write_multiple_registers ( self.last_transaction_id, 
																								   self.unit_identifier, 
																								   starting_address, 
																								   register_values );

		if l_request.is_ok ( )
		{
			let mut l_request_telegram : Option<ModbusTelegram> = Some ( l_request.unwrap ( ) );

			let l_stream : Option<TcpStream> = self.stream.take ( );

			if l_stream.is_some ( )
			{
				let mut l_tcp : TcpStream = l_stream.unwrap ( );
			
				let l_response_telegram : Option<ModbusTelegram> = process_modbus_telegram ( &mut l_tcp, 
																							 &mut l_request_telegram );

				self.stream = Some ( l_tcp );
				self.update_last_transaction_id ( );

				if l_response_telegram.is_some ( )
				{
					let l_request : ModbusTelegram = l_request_telegram.unwrap ( );
					let l_response : ModbusTelegram = l_response_telegram.unwrap ( );

					//	check function_code
					if verify_function_code ( &l_request, &l_response )
					{
						//	prepare response data
						let l_response_data : Vec<u16> = prepare_response_write_multiple_registers ( &l_response.get_payload ( ).unwrap ( ) );

                        if l_response_data.len ( ) > 0
                        {
                            //	PERFORMANCE-Info
                            let l_elapsed_time : Duration = l_start_time.elapsed ( ).unwrap ( );
                            let l_milliseconds : u64 = compute_milliseconds ( &l_elapsed_time );

                            l_return = ModbusReturnRegisters::Good ( ReturnGood::new ( l_response_data, l_milliseconds ) );
                        }
                        else
                        {
                            l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "modbus response data is invalid" ) );
                        }
					}
					else
					{
						l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_codes ( l_response.get_function_code ( ).unwrap ( ), 1 ) );
					}					
				}
				else
				{
					l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "created modbus telegram is invalid" ) );
				}
			}
			else
			{
				l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( "tcp client is invalid" ) );
			}
		}
		else
		{
			l_return = ModbusReturnRegisters::Bad ( ReturnBad::new_with_message ( &l_request.err ( ).unwrap ( ) ) );
		}

		return l_return;
	}

}
