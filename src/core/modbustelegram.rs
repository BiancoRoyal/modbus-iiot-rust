

use core::datatransformation::*;


const MODBUS_FUNCTION_CODE_LENGTH : u16 = 0x0001;
const MODBUS_HEADER_SIZE : u8 = 0x07;
const MODBUS_PROTOCOL_IDENTIFIER_TCP : u16 = 0x0000;
const MODBUS_UNIT_IDENTIFIER_LENGTH : u16 = 0x0001;


pub struct ModbusTelegram
{
	transaction_identifier : u16,

	unit_identifier : u8,

	function_code : u8,

	payload : Vec<u8>,

	expected_bytes : u8
}


impl ModbusTelegram
{
	pub fn new ( transaction_identifier : u16, unit_identifier : u8, function_code : u8, payload : &Vec<u8>, expected_bytes : u8 ) -> Option<ModbusTelegram>
	{
		let l_return : Option<ModbusTelegram>;

		if transaction_identifier > 0x0000 && function_code > 0x00
		{
			l_return = 
				Some
				(
					ModbusTelegram
					{
						transaction_identifier : transaction_identifier,
						unit_identifier : unit_identifier,
						function_code : function_code,
						payload : payload.clone ( ),
						expected_bytes : expected_bytes
					}
				);
		}
		else
		{		
			l_return = None;		
		}

		return l_return;
	}

	pub fn new_from_bytes ( bytes : &Vec<u8> ) ->Option<ModbusTelegram>
	{
		let l_return : Option<ModbusTelegram>;

		if bytes.len ( ) > 9
		{
			let l_response_transaction_identifier : Option<u16> = extract_word_from_bytearray ( &bytes, 0 );
			let l_response_unit_identifier : Option<u8> = extract_byte_from_bytearray ( &bytes, 6 );
			let l_response_function_code : Option<u8> =	extract_byte_from_bytearray ( &bytes, 7 );
			let l_function_code : u8 = l_response_function_code.unwrap ( );
			let l_response_payload : Option<Vec<u8>> = extract_payload_by_function_code ( l_function_code, &bytes );

			if l_response_transaction_identifier.is_some ( ) &&
			   l_response_unit_identifier.is_some ( ) &&
			   l_response_payload.is_some ( )
			{
				l_return =
					Some
					(
						ModbusTelegram
						{
							transaction_identifier : l_response_transaction_identifier.unwrap ( ),
							unit_identifier : l_response_unit_identifier.unwrap ( ),
							function_code :	l_function_code,
							payload : l_response_payload.unwrap ( ),
							expected_bytes : 0x00
						}
					);
			}
			else
			{			
				l_return = None;
			}
		}
		else
		{
			l_return = None;
		}

		return l_return;
	}

	pub fn get_bytes ( &self ) -> Option<Vec<u8>>
	{
		let mut l_return : Vec<u8> = vec![];

		let l_length_for_header : u16 =	self.payload.len ( ) as u16 + MODBUS_UNIT_IDENTIFIER_LENGTH + MODBUS_FUNCTION_CODE_LENGTH;

		append_word_to_bytearray ( &mut l_return, self.transaction_identifier );
		
		append_word_to_bytearray ( &mut l_return, MODBUS_PROTOCOL_IDENTIFIER_TCP );

		append_word_to_bytearray ( &mut l_return, l_length_for_header );

		append_byte_to_bytearray ( &mut l_return, self.unit_identifier );

		append_byte_to_bytearray ( &mut l_return, self.function_code );

		append_bytearray_to_bytearray ( &mut l_return, &self.payload );

		return Some ( l_return );
	}

	pub fn get_expected_byte_count ( &self ) -> Option<u8>
	{
		let l_return : Option<u8>;

		if self.expected_bytes > MODBUS_HEADER_SIZE
		{
			l_return = Some ( self.expected_bytes );			
		}
		else
		{
			l_return = None;
		}

		return l_return;
	}

	pub fn get_function_code ( &self ) -> Option<u8>
	{
		let l_return : Option<u8>;

		if self.function_code > 0x00
		{
			l_return = Some ( self.function_code );			
		}
		else
		{
			l_return = None;
		}

		return l_return;
	}

	pub fn get_payload ( &self ) -> Option<Vec<u8>>
	{
		return Some ( self.payload.clone ( ) );
	}
}

//	===============================================================================================

#[test]
fn test_extract_payload_by_function_code ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 ) ;	//	transaction_identifier
	l_test_1_data.push ( 0xA0 );	//	transaction_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	length of all following bytes
	l_test_1_data.push ( 0x09 );	//	length of all following bytes
	l_test_1_data.push ( 0x01 );	//	unit_identifier
	l_test_1_data.push ( 0x03 );	//	FUNCTION_CODE_READ_HOLDING_REGISTERS
	l_test_1_data.push ( 0x06 );	//	byte count
	l_test_1_data.push ( 0xF0 );	//	data
	l_test_1_data.push ( 0x0F );	//	data
	l_test_1_data.push ( 0x00 );	//	data
	l_test_1_data.push ( 0xFF );	//	data
	l_test_1_data.push ( 0xFF );	//	data
	l_test_1_data.push ( 0x00 );	//	data

	let l_result_1_data : Option<Vec<u8>> = extract_payload_by_function_code ( 0x03, &l_test_1_data );

	assert_eq! ( l_result_1_data.is_some ( ), true );

	let l_result_1_bytes : Vec<u8> = l_result_1_data.unwrap ( );

	assert_eq! ( l_result_1_bytes.len ( ), 7 );

	assert_eq! ( l_result_1_bytes[0], 0x06 );
	assert_eq! ( l_result_1_bytes[1], 0xF0 );
	assert_eq! ( l_result_1_bytes[2], 0x0F );
	assert_eq! ( l_result_1_bytes[3], 0x00 );
	assert_eq! ( l_result_1_bytes[4], 0xFF );
	assert_eq! ( l_result_1_bytes[5], 0xFF );
	assert_eq! ( l_result_1_bytes[6], 0x00 );


	let mut l_test_2_data : Vec<u8> = vec![];

	l_test_2_data.push ( 0x00 ) ;	//	transaction_identifier
	l_test_2_data.push ( 0xA0 );	//	transaction_identifier
	l_test_2_data.push ( 0x00 );	//	protocol_identifier
	l_test_2_data.push ( 0x00 );	//	protocol_identifier
	l_test_2_data.push ( 0x00 );	//	length of all following bytes
	l_test_2_data.push ( 0x06 );	//	length of all following bytes
	l_test_2_data.push ( 0x01 );	//	unit_identifier
	l_test_2_data.push ( 0x10 );	//	FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS
	l_test_2_data.push ( 0x01 );	//	starting address
	l_test_2_data.push ( 0x00 );	//	starting address
	l_test_2_data.push ( 0x00 );	//	quantity_of_registers
	l_test_2_data.push ( 0x10 );	//	quantity_of_registers
	
	let l_result_2_data : Option<Vec<u8>> = extract_payload_by_function_code ( 0x10, &l_test_2_data );

	assert_eq! ( l_result_2_data.is_some ( ), true );

	let l_result_2_bytes : Vec<u8> = l_result_2_data.unwrap ( );

	assert_eq! ( l_result_2_bytes.len ( ), 4 );

	assert_eq! ( l_result_2_bytes[0], 0x01 );
	assert_eq! ( l_result_2_bytes[1], 0x00 );
	assert_eq! ( l_result_2_bytes[2], 0x00 );
	assert_eq! ( l_result_2_bytes[3], 0x10 );	
}

fn extract_payload_by_function_code ( function_code : u8, bytes : &Vec<u8> ) -> Option<Vec<u8>>
{
	let l_return : Option<Vec<u8>>;

	println! ( "debug - function code is {}", function_code );

	match function_code
	{
		0x01	=> { l_return = extract_payload_with_byte_count ( &bytes ); }
		0x02	=> { l_return = extract_payload_with_byte_count ( &bytes ); }
		0x03	=> { l_return = extract_payload_with_byte_count ( &bytes ); }
		0x04	=> { l_return = extract_payload_with_byte_count ( &bytes ); }
		0x05	=> { l_return = extract_payload_without_byte_count ( &bytes ); }
		0x06	=> { l_return = extract_payload_without_byte_count ( &bytes ); }
		0x0F	=> { l_return = extract_payload_without_byte_count ( &bytes ); }
		0x10	=> { l_return = extract_payload_without_byte_count ( &bytes ); }
		_		=> { l_return = None; }
	}

	if l_return.is_some ( )
	{
		println! ( "debug - payload is some" );
	}
	else
	{
		println! ( "debug - payload is none" );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_extract_payload_with_byte_count ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 ) ;	//	transaction_identifier
	l_test_1_data.push ( 0xA0 );	//	transaction_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	length of all following bytes
	l_test_1_data.push ( 0x09 );	//	length of all following bytes
	l_test_1_data.push ( 0x01 );	//	unit_identifier
	l_test_1_data.push ( 0x03 );	//	FUNCTION_CODE_READ_HOLDING_REGISTERS
	l_test_1_data.push ( 0x06 );	//	byte count
	l_test_1_data.push ( 0xF0 );	//	data
	l_test_1_data.push ( 0x0F );	//	data
	l_test_1_data.push ( 0x00 );	//	data
	l_test_1_data.push ( 0xFF );	//	data
	l_test_1_data.push ( 0xFF );	//	data
	l_test_1_data.push ( 0x00 );	//	data

	let l_result_1_data : Option<Vec<u8>> = extract_payload_with_byte_count ( &l_test_1_data );

	assert_eq! ( l_result_1_data.is_some ( ), true );

	let l_result_bytes : Vec<u8> = l_result_1_data.unwrap ( );

	assert_eq! ( l_result_bytes.len ( ), 7 );

	assert_eq! ( l_result_bytes[0], 0x06 );
	assert_eq! ( l_result_bytes[1], 0xF0 );
	assert_eq! ( l_result_bytes[2], 0x0F );
	assert_eq! ( l_result_bytes[3], 0x00 );
	assert_eq! ( l_result_bytes[4], 0xFF );
	assert_eq! ( l_result_bytes[5], 0xFF );
	assert_eq! ( l_result_bytes[6], 0x00 );
}

fn extract_payload_with_byte_count ( bytes : &Vec<u8> ) -> Option<Vec<u8>>
{
	let l_return : Option<Vec<u8>>;

	let l_byte_count : Option<u8> = extract_byte_from_bytearray ( &bytes, 8 );
	let l_bytes : u8 = l_byte_count.unwrap ( ) + 1;

	l_return = extract_bytes_from_bytearray ( &bytes, 8, l_bytes );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_extract_payload_without_byte_count ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 ) ;	//	transaction_identifier
	l_test_1_data.push ( 0xA0 );	//	transaction_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	protocol_identifier
	l_test_1_data.push ( 0x00 );	//	length of all following bytes
	l_test_1_data.push ( 0x06 );	//	length of all following bytes
	l_test_1_data.push ( 0x01 );	//	unit_identifier
	l_test_1_data.push ( 0x10 );	//	FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS
	l_test_1_data.push ( 0x01 );	//	starting address
	l_test_1_data.push ( 0x00 );	//	starting address
	l_test_1_data.push ( 0x00 );	//	quantity_of_registers
	l_test_1_data.push ( 0x10 );	//	quantity_of_registers
	
	let l_result_1_data : Option<Vec<u8>> = extract_payload_without_byte_count ( &l_test_1_data );

	assert_eq! ( l_result_1_data.is_some ( ), true );

	let l_result_bytes : Vec<u8> = l_result_1_data.unwrap ( );

	assert_eq! ( l_result_bytes.len ( ), 4 );

	assert_eq! ( l_result_bytes[0], 0x01 );
	assert_eq! ( l_result_bytes[1], 0x00 );
	assert_eq! ( l_result_bytes[2], 0x00 );
	assert_eq! ( l_result_bytes[3], 0x10 );
}

fn extract_payload_without_byte_count ( bytes : &Vec<u8> ) -> Option<Vec<u8>>
{
	let l_return : Option<Vec<u8>>;

	let l_byte_count : u8 = bytes.len ( ) as u8 - MODBUS_HEADER_SIZE - 1; // -1 for FunctionCode

	l_return = extract_bytes_from_bytearray ( &bytes, 8, l_byte_count );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_function_code ( )
{
	let mut l_payload : Vec<u8> = vec![];
	l_payload.push ( 0x00 );
	l_payload.push ( 0xFF );
	l_payload.push ( 0x00 );
	l_payload.push ( 0x0A );

	let l_test_data_request : Option<ModbusTelegram> = ModbusTelegram::new ( 0x00A0, 0x01, 0x01, &l_payload, 0x00 );

	assert_eq! ( l_test_data_request.is_some ( ), true );
	

	let l_test_data_response : Option<ModbusTelegram> = ModbusTelegram::new ( 0x00A0, 0x01, 0x01, &l_payload, 0x00 );
	
	assert_eq! ( l_test_data_response.is_some ( ), true );


	let l_is_equal : bool = verify_function_code ( &l_test_data_request.unwrap ( ), &l_test_data_response.unwrap ( ) );

	assert_eq! ( l_is_equal, true );
}

pub fn verify_function_code ( request_telegram : &ModbusTelegram, response_telegram : &ModbusTelegram ) -> bool
{
	let l_return : bool;

	let l_function_code_request_option : Option<u8> = request_telegram.get_function_code ( );
	let l_function_code_response_option : Option<u8> = response_telegram.get_function_code ( );

	if l_function_code_request_option.is_some ( ) &&
	   l_function_code_response_option.is_some ( )
	{
		let l_function_code_request : u8 = l_function_code_request_option.unwrap ( );
		let l_function_code_response : u8 =	l_function_code_response_option.unwrap ( );

		if l_function_code_request == l_function_code_response
		{
			l_return = true;
		}
		else
		{
			l_return = false;
		}
	}
	else
	{
		l_return = false;
	}

	return l_return;
}
