

use core::consts::*;
use core::datatransformation::*;

//	===============================================================================================

pub struct ModbusTelegram
{
	transaction_identifier : u16,
	unit_identifier : u8,
	function_code : u8,
	payload : Vec< u8 >,
	expected_bytes : u8
}

impl ModbusTelegram
{
	pub fn new ( transaction_identifier : u16, unit_identifier : u8, function_code : u8, payload : &Vec< u8 >, expected_bytes : u8 ) -> Option< ModbusTelegram >
	{
		let reply : Option< ModbusTelegram >;

		if transaction_identifier > 0x0000 && function_code > 0x00
		{
			reply = Some(
				ModbusTelegram
				{
					transaction_identifier : transaction_identifier,
					unit_identifier : unit_identifier,
					function_code : function_code,
					payload : payload.clone (),
					expected_bytes : expected_bytes
				}
			);
		}
		else
		{		
			reply = None;		
		}

		return reply;
	}

	pub fn new_from_bytes ( bytes : &Vec< u8 > ) -> Option< ModbusTelegram >
	{
		let reply : Option< ModbusTelegram >;

		if bytes.len () > 9
		{
			let response_transaction_identifier : Option< u16 > = extract_word_from_bytearray ( &bytes, 
																								0 );
			let response_unit_identifier : Option< u8 > = extract_byte_from_bytearray ( &bytes, 
																						6 );
			let response_function_code : Option< u8 > =	extract_byte_from_bytearray ( &bytes, 
																					  7 );
			let function_code : u8 = response_function_code.unwrap ();
			let response_payload : Option< Vec< u8 > > = extract_payload_by_function_code ( function_code, 
																							&bytes );

			if response_transaction_identifier.is_some () &&
			   response_unit_identifier.is_some () &&
			   response_payload.is_some ()
			{
				reply =	Some(
					ModbusTelegram
					{
						transaction_identifier : response_transaction_identifier.unwrap (),
						unit_identifier : response_unit_identifier.unwrap (),
						function_code :	function_code,
						payload : response_payload.unwrap (),
						expected_bytes : 0x00
					}
				);
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

	pub fn get_bytes ( &self ) -> Option< Vec< u8 > >
	{
		let mut reply : Vec< u8 > = vec![];

		let length_for_header : u16 = self.payload.len () as u16 + MODBUS_UNIT_IDENTIFIER_LENGTH + MODBUS_FUNCTION_CODE_LENGTH;

		append_word_to_bytearray ( &mut reply, 
								   self.transaction_identifier );
		append_word_to_bytearray ( &mut reply, 
								   MODBUS_PROTOCOL_IDENTIFIER_TCP );
		append_word_to_bytearray ( &mut reply, 
								   length_for_header );
		append_byte_to_bytearray ( &mut reply, 
								   self.unit_identifier );
		append_byte_to_bytearray ( &mut reply, 
								   self.function_code );
		append_bytearray_to_bytearray ( &mut reply, 
										&self.payload );

		return Some( reply );
	}

	pub fn get_expected_byte_count ( &self ) -> Option< u8 >
	{
		let reply : Option< u8 >;

		if self.expected_bytes > MODBUS_HEADER_SIZE
		{
			reply = Some( self.expected_bytes );			
		}
		else
		{
			reply = None;
		}

		return reply;
	}

	pub fn get_function_code ( &self ) -> Option< u8 >
	{
		let reply : Option< u8 >;

		if self.function_code > 0x00
		{
			reply = Some( self.function_code );			
		}
		else
		{
			reply = None;
		}

		return reply;
	}

	pub fn get_payload ( &self ) -> Option< Vec< u8 > >
	{
		return Some( self.payload.clone () );
	}
}

//	===============================================================================================

#[test]
fn test_extract_payload_by_function_code ()
{
	let mut test_data_1 : Vec< u8 > = vec![];
	test_data_1.push ( 0x00 ) ;	//	transaction_identifier
	test_data_1.push ( 0xA0 );	//	transaction_identifier
	test_data_1.push ( 0x00 );	//	protocol_identifier
	test_data_1.push ( 0x00 );	//	protocol_identifier
	test_data_1.push ( 0x00 );	//	length of all following bytes
	test_data_1.push ( 0x09 );	//	length of all following bytes
	test_data_1.push ( 0x01 );	//	unit_identifier
	test_data_1.push ( 0x03 );	//	FUNCTION_CODE_READ_HOLDING_REGISTERS
	test_data_1.push ( 0x06 );	//	byte count
	test_data_1.push ( 0xF0 );	//	data
	test_data_1.push ( 0x0F );	//	data
	test_data_1.push ( 0x00 );	//	data
	test_data_1.push ( 0xFF );	//	data
	test_data_1.push ( 0xFF );	//	data
	test_data_1.push ( 0x00 );	//	data

	let result_data_1 : Option< Vec< u8 > > = extract_payload_by_function_code ( 0x03, 
																				 &test_data_1 );
	assert! ( result_data_1.is_some () );

	let result_bytes_1 : Vec< u8 > = result_data_1.unwrap ();
	assert_eq! ( result_bytes_1.len (), 7 );
	assert_eq! ( result_bytes_1[ 0 ], 0x06 );
	assert_eq! ( result_bytes_1[ 1 ], 0xF0 );
	assert_eq! ( result_bytes_1[ 2 ], 0x0F );
	assert_eq! ( result_bytes_1[ 3 ], 0x00 );
	assert_eq! ( result_bytes_1[ 4 ], 0xFF );
	assert_eq! ( result_bytes_1[ 5 ], 0xFF );
	assert_eq! ( result_bytes_1[ 6 ], 0x00 );

	let mut test_data_2 : Vec< u8 > = vec![];
	test_data_2.push ( 0x00 );	//	transaction_identifier
	test_data_2.push ( 0xA0 );	//	transaction_identifier
	test_data_2.push ( 0x00 );	//	protocol_identifier
	test_data_2.push ( 0x00 );	//	protocol_identifier
	test_data_2.push ( 0x00 );	//	length of all following bytes
	test_data_2.push ( 0x06 );	//	length of all following bytes
	test_data_2.push ( 0x01 );	//	unit_identifier
	test_data_2.push ( 0x10 );	//	FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS
	test_data_2.push ( 0x01 );	//	starting address
	test_data_2.push ( 0x00 );	//	starting address
	test_data_2.push ( 0x00 );	//	quantity_of_registers
	test_data_2.push ( 0x10 );	//	quantity_of_registers
	
	let result_data_2 : Option< Vec< u8 > > = extract_payload_by_function_code ( 0x10, 
																				 &test_data_2 );
	assert! ( result_data_2.is_some () );

	let result_bytes_2 : Vec< u8 > = result_data_2.unwrap ();
	assert_eq! ( result_bytes_2.len (), 4 );
	assert_eq! ( result_bytes_2[ 0 ], 0x01 );
	assert_eq! ( result_bytes_2[ 1 ], 0x00 );
	assert_eq! ( result_bytes_2[ 2 ], 0x00 );
	assert_eq! ( result_bytes_2[ 3 ], 0x10 );	
}

fn extract_payload_by_function_code ( function_code : u8, bytes : &Vec< u8 > ) -> Option< Vec< u8 > >
{
	let reply : Option< Vec< u8 > >;

	match function_code
	{
		0x01	=> { reply = extract_payload_with_byte_count ( &bytes ); }
		0x02	=> { reply = extract_payload_with_byte_count ( &bytes ); }
		0x03	=> { reply = extract_payload_with_byte_count ( &bytes ); }
		0x04	=> { reply = extract_payload_with_byte_count ( &bytes ); }
		0x05	=> { reply = extract_payload_without_byte_count ( &bytes ); }
		0x06	=> { reply = extract_payload_without_byte_count ( &bytes ); }
		0x0F	=> { reply = extract_payload_without_byte_count ( &bytes ); }
		0x10	=> { reply = extract_payload_without_byte_count ( &bytes ); }
		_		=> { reply = None; }
	}

	return reply;
}

//	===============================================================================================

#[test]
fn test_extract_payload_with_byte_count ()
{
	let mut test_data : Vec< u8 > = vec![];

	test_data.push ( 0x00 );	//	transaction_identifier
	test_data.push ( 0xA0 );	//	transaction_identifier
	test_data.push ( 0x00 );	//	protocol_identifier
	test_data.push ( 0x00 );	//	protocol_identifier
	test_data.push ( 0x00 );	//	length of all following bytes
	test_data.push ( 0x09 );	//	length of all following bytes
	test_data.push ( 0x01 );	//	unit_identifier
	test_data.push ( 0x03 );	//	FUNCTION_CODE_READ_HOLDING_REGISTERS
	test_data.push ( 0x06 );	//	byte count
	test_data.push ( 0xF0 );	//	data
	test_data.push ( 0x0F );	//	data
	test_data.push ( 0x00 );	//	data
	test_data.push ( 0xFF );	//	data
	test_data.push ( 0xFF );	//	data
	test_data.push ( 0x00 );	//	data

	let result_data : Option< Vec< u8 > > = extract_payload_with_byte_count ( &test_data );
	assert! ( result_data.is_some () );

	let result_bytes : Vec< u8 > = result_data.unwrap ();
	assert_eq! ( result_bytes.len (), 7 );
	assert_eq! ( result_bytes[ 0 ], 0x06 );
	assert_eq! ( result_bytes[ 1 ], 0xF0 );
	assert_eq! ( result_bytes[ 2 ], 0x0F );
	assert_eq! ( result_bytes[ 3 ], 0x00 );
	assert_eq! ( result_bytes[ 4 ], 0xFF );
	assert_eq! ( result_bytes[ 5 ], 0xFF );
	assert_eq! ( result_bytes[ 6 ], 0x00 );
}

fn extract_payload_with_byte_count ( bytes : &Vec< u8 > ) -> Option< Vec< u8 > >
{
	let reply : Option< Vec< u8 > >;

	let byte_count : Option< u8 > = extract_byte_from_bytearray ( &bytes, 
																  8 );
	let count : u8 = byte_count.unwrap () + 1;

	reply = extract_bytes_from_bytearray ( &bytes, 
										   8, 
										   count );

	return reply;
}

//	===============================================================================================

#[test]
fn test_extract_payload_without_byte_count ()
{
	let mut test_data : Vec< u8 > = vec![];

	test_data.push ( 0x00 ) ;	//	transaction_identifier
	test_data.push ( 0xA0 );	//	transaction_identifier
	test_data.push ( 0x00 );	//	protocol_identifier
	test_data.push ( 0x00 );	//	protocol_identifier
	test_data.push ( 0x00 );	//	length of all following bytes
	test_data.push ( 0x06 );	//	length of all following bytes
	test_data.push ( 0x01 );	//	unit_identifier
	test_data.push ( 0x10 );	//	FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS
	test_data.push ( 0x01 );	//	starting address
	test_data.push ( 0x00 );	//	starting address
	test_data.push ( 0x00 );	//	quantity_of_registers
	test_data.push ( 0x10 );	//	quantity_of_registers
	
	let result_data : Option< Vec< u8 > > = extract_payload_without_byte_count ( &test_data );
	assert! ( result_data.is_some () );

	let result_bytes : Vec< u8 > = result_data.unwrap ();
	assert_eq! ( result_bytes.len (), 4 );
	assert_eq! ( result_bytes[ 0 ], 0x01 );
	assert_eq! ( result_bytes[ 1 ], 0x00 );
	assert_eq! ( result_bytes[ 2 ], 0x00 );
	assert_eq! ( result_bytes[ 3 ], 0x10 );
}

fn extract_payload_without_byte_count ( bytes : &Vec< u8 > ) -> Option< Vec< u8 > >
{
	let reply : Option< Vec< u8 > >;

	let byte_count : u8 = bytes.len () as u8 - MODBUS_HEADER_SIZE - 1; // -1 for FunctionCode

	reply = extract_bytes_from_bytearray ( &bytes, 
										   8, 
										   byte_count );

	return reply;
}

//	===============================================================================================

#[test]
fn test_verify_function_code ()
{
	let l_payload : Vec< u8 > = vec![ 0x00, 0xFF, 0x00, 0x0A ];

	let test_data_request : Option< ModbusTelegram > = ModbusTelegram::new ( 0x00A0, 
																			 0x01, 
																			 0x01, 
																			 &l_payload, 
																			 0x00 );
	assert! ( test_data_request.is_some () );
	
	let test_data_response : Option< ModbusTelegram > = ModbusTelegram::new ( 0x00A0, 
																			  0x01, 
																			  0x01, 
																			  &l_payload, 
																			  0x00 );
	assert! ( test_data_response.is_some () );

	let is_equal : bool = verify_function_code ( &test_data_request.unwrap (), 
												 &test_data_response.unwrap () );
	assert! ( is_equal );
}

pub fn verify_function_code ( request_telegram : &ModbusTelegram, response_telegram : &ModbusTelegram ) -> bool
{
	let mut reply : bool = false;

	if let Some( function_code_request ) = request_telegram.get_function_code ()
	{
	    if let Some( function_code_response ) = response_telegram.get_function_code ()
		{
			if function_code_request == function_code_response
			{
				reply = true;
			}
		}
	}

	return reply;
}
