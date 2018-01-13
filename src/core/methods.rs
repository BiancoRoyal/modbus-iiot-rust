

use core::datatransformation::*;
use core::modbustelegram::ModbusTelegram;

const FUNCTION_CODE_READ_COILS : u8 = 0x01;
const FUNCTION_CODE_READ_DISCRETE_INPUTS : u8 = 0x02;
const FUNCTION_CODE_READ_HOLDING_REGISTERS : u8 = 0x03;
const FUNCTION_CODE_READ_INPUT_REGISTERS : u8 = 0x04;
const FUNCTION_CODE_WRITE_MULTIPLE_COILS : u8 = 0x0F;
const FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS : u8 = 0x10;
const FUNCTION_CODE_WRITE_SINGLE_COIL : u8 = 0x05;
const FUNCTION_CODE_WRITE_SINGLE_REGISTER : u8 = 0x06;

const MODBUS_HEADER_SIZE : u8 = 0x07;

//	in BETA-Version aufteilen in eine Datei pro Function-Code

//	===============================================================================================

#[test]
fn test_create_request_read_coils ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;
	let l_quantity_of_coils : u16 = 0x000A;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_read_coils ( l_transaction_identifier,
																					   l_unit_identifier,
																					   l_starting_address,
																					   l_quantity_of_coils );

	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_READ_COILS );

	
	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> =	l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_READ_COILS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_coils
	assert_eq! ( l_bytes[11], 0x0A );	//	quantity_of_coils
}

pub fn create_request_read_coils ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, quantity_of_coils : u16 ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_read_coils ( starting_address, quantity_of_coils );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_read_coils ( starting_address, quantity_of_coils );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
										    							unit_identifier,
																		FUNCTION_CODE_READ_COILS,
																		&l_payload,
																		get_expected_byte_count_read_coils ( quantity_of_coils ) 
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_read_discrete_inputs ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;
	let l_quantity_of_inputs : u16 = 0x000A;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_read_discrete_inputs ( l_transaction_identifier,
																								 l_unit_identifier,
																								 l_starting_address,
																								 l_quantity_of_inputs
																							   );


	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_READ_DISCRETE_INPUTS );

	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> =	l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_READ_DISCRETE_INPUTS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_inputs
	assert_eq! ( l_bytes[11], 0x0A );	//	quantity_of_inputs
}

pub fn create_request_read_discrete_inputs ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, quantity_of_inputs : u16 ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_read_discrete_inputs ( starting_address, quantity_of_inputs );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_read_discrete_inputs ( starting_address, quantity_of_inputs );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_READ_DISCRETE_INPUTS,
																		&l_payload,
																		get_expected_byte_count_read_discrete_inputs ( quantity_of_inputs )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_read_holding_registers ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;
	let l_quantity_of_registers : u16 = 0x000A;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_read_holding_registers ( l_transaction_identifier,
																								   l_unit_identifier,
																								   l_starting_address,
																								   l_quantity_of_registers
																								 );

	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_READ_HOLDING_REGISTERS );

	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> = l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_READ_HOLDING_REGISTERS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_registers
	assert_eq! ( l_bytes[11], 0x0A );	//	quantity_of_registers
}

pub fn create_request_read_holding_registers ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, quantity_of_registers : u16 ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_read_holding_registers ( starting_address, quantity_of_registers );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_read_holding_registers ( starting_address, quantity_of_registers );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_READ_HOLDING_REGISTERS,
																		&l_payload,
																		get_expected_byte_count_read_holding_registers ( quantity_of_registers )
																	  );

		l_return = pack_telegram ( l_telegram );		
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_read_input_registers ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;
	let l_quantity_of_input_registers : u16 = 0x000A;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_read_input_registers ( l_transaction_identifier,
																								 l_unit_identifier,
																								 l_starting_address,
																								 l_quantity_of_input_registers
																							   );

	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_READ_INPUT_REGISTERS );

	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> =	l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_READ_INPUT_REGISTERS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_input_registers
	assert_eq! ( l_bytes[11], 0x0A );	//	quantity_of_input_registers
}

pub fn create_request_read_input_registers ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, quantity_of_input_registers : u16 ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_read_input_registers ( starting_address, quantity_of_input_registers );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_read_input_registers ( starting_address, quantity_of_input_registers );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_READ_INPUT_REGISTERS,
																		&l_payload,
																		get_expected_byte_count_read_input_registers ( quantity_of_input_registers )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_write_multiple_coils ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;
	let l_quantity_of_outputs : u16 = 18;

	let mut l_output_values : Vec<u8> = vec![];

	l_output_values.push ( 0xFF );
	l_output_values.push ( 0xF0 );
	l_output_values.push ( 0x02 );	


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_write_multiple_coils ( l_transaction_identifier,
																								 l_unit_identifier,
																								 l_starting_address, 
																								 l_quantity_of_outputs,
																								 l_output_values
																							   );

	assert_eq! ( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_WRITE_MULTIPLE_COILS );

	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> = l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 16 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x0A );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_WRITE_MULTIPLE_COILS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_outputs
	assert_eq! ( l_bytes[11], 0x12 );	//	quantity_of_outputs
	assert_eq! ( l_bytes[12], 0x03 );	//	byte_count
	assert_eq! ( l_bytes[13], 0xFF );	//	output_value
	assert_eq! ( l_bytes[14], 0xF0 );	//	output_value
	assert_eq! ( l_bytes[15], 0x02 );	//	output_value	
}

pub fn create_request_write_multiple_coils ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, quantity_of_outputs : u16, output_values : Vec<u8> ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_write_multiple_coils ( starting_address, quantity_of_outputs );
 
	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_write_multiple_coils ( starting_address, quantity_of_outputs, &output_values );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_WRITE_MULTIPLE_COILS,
																		&l_payload,
																		get_expected_byte_count_write_multiple_coils ( )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_write_multiple_registers ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_starting_address : u16 = 0x00FF;

	let mut l_register_values : Vec<u16> = vec![];

	l_register_values.push ( 0x00FF );
	l_register_values.push ( 0xF00F );
	l_register_values.push ( 0x010A );
	l_register_values.push ( 0xABCD );


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_write_multiple_registers ( l_transaction_identifier,
																								     l_unit_identifier,
																									 l_starting_address, 
																									 l_register_values
																								   );

	assert_eq! ( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS );

	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> = l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 21 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x0F );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	starting_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	starting_address
	assert_eq! ( l_bytes[10], 0x00 );	//	quantity_of_registers
	assert_eq! ( l_bytes[11], 0x04 );	//	quantity_of_registers
	assert_eq! ( l_bytes[12], 0x08 );	//	byte_count
	assert_eq! ( l_bytes[13], 0x00 );	//	register_value
	assert_eq! ( l_bytes[14], 0xFF );	//	register_value
	assert_eq! ( l_bytes[15], 0xF0 );	//	register_value
	assert_eq! ( l_bytes[16], 0x0F );	//	register_value
	assert_eq! ( l_bytes[17], 0x01 );	//	register_value
	assert_eq! ( l_bytes[18], 0x0A );	//	register_value
	assert_eq! ( l_bytes[19], 0xAB );	//	register_value
	assert_eq! ( l_bytes[20], 0xCD );	//	register_value
}

pub fn create_request_write_multiple_registers ( transaction_identifier : u16, unit_identifier : u8, starting_address : u16, register_values : Vec<u16> ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_quantity_of_registers : u16 =	register_values.len ( ) as u16;

	let l_verify : Result<bool, String> = verify_parameter_write_multiple_registers ( starting_address, l_quantity_of_registers );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_write_multiple_registers ( starting_address, &register_values );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS,
																		&l_payload,
																		get_expected_byte_count_write_multiple_registers ( )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_write_single_coil ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_output_address : u16 = 0x00FF;
	let l_output_value : u16 = 0xFF00;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_write_single_coil ( l_transaction_identifier,
																							  l_unit_identifier,
																							  l_output_address,
																							  l_output_value
																							);

	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_WRITE_SINGLE_COIL );
	
	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> = l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_WRITE_SINGLE_COIL );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	output_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	output_address
	assert_eq! ( l_bytes[10], 0xFF );	//	output_value
	assert_eq! ( l_bytes[11], 0x00 );	//	output_value
}

pub fn create_request_write_single_coil ( transaction_identifier : u16, unit_identifier : u8, output_address : u16, output_value : u16 ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_write_single_coil ( output_value );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_write_single_coil ( output_address, output_value );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_WRITE_SINGLE_COIL,
																		&l_payload,
																		get_expected_byte_count_write_single_coil ( )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_create_request_write_single_register ( )
{
	let l_transaction_identifier : u16 = 0x00A0;
	let l_unit_identifier : u8 = 0x01;
	let l_register_address : u16 = 0x00FF;
	let l_register_value : u16 = 0xF0F0;


	let l_result_1_data : Result<ModbusTelegram, String> = create_request_write_single_register ( l_transaction_identifier,
																								  l_unit_identifier,
																								  l_register_address,
																								  l_register_value
																								);

	assert_eq!( l_result_1_data.is_ok ( ), true );

	let l_telegram : ModbusTelegram = l_result_1_data.unwrap ( );
	
	//	den FunctionCode von ModbusTelegrams testen
	let l_function_code: Option<u8> = l_telegram.get_function_code ( );
	
	assert_eq! ( l_function_code.is_some ( ), true );
	assert_eq! ( l_function_code.unwrap ( ), FUNCTION_CODE_WRITE_SINGLE_REGISTER );
	
	//	alle bytes von ModbusTelegram testen
	let l_telegram_bytes : Option<Vec<u8>> = l_telegram.get_bytes ( );

	assert_eq! ( l_telegram_bytes.is_some ( ), true );

	let l_bytes : Vec<u8> =	l_telegram_bytes.unwrap ( );

	assert_eq! ( l_bytes.len ( ), 12 );

	assert_eq! ( l_bytes[ 0], 0x00 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 1], 0xA0 );	//	transaction_identifier
	assert_eq! ( l_bytes[ 2], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 3], 0x00 );	//	protocol_identifier
	assert_eq! ( l_bytes[ 4], 0x00 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 5], 0x06 );	//	length of all following bytes
	assert_eq! ( l_bytes[ 6], l_unit_identifier );
	assert_eq! ( l_bytes[ 7], FUNCTION_CODE_WRITE_SINGLE_REGISTER );
	assert_eq! ( l_bytes[ 8], 0x00 );	//	register_address
	assert_eq! ( l_bytes[ 9], 0xFF );	//	register_address
	assert_eq! ( l_bytes[10], 0xF0 );	//	register_value
	assert_eq! ( l_bytes[11], 0xF0 );	//	register_value	
}

pub fn create_request_write_single_register ( transaction_identifier : u16, unit_identifier : u8, register_address : u16, register_value : u16 ) -> Result<ModbusTelegram, String>
{	
	let l_return : Result<ModbusTelegram, String>;

	let l_verify : Result<bool, String> = verify_parameter_write_single_register ( );

	if l_verify.is_ok ( )
	{
		let l_payload : Vec<u8> = prepare_payload_write_single_register ( register_address, register_value );

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new ( transaction_identifier,
																		unit_identifier,
																		FUNCTION_CODE_WRITE_SINGLE_REGISTER,
																		&l_payload,
																		get_expected_byte_count_write_single_register ( )
																	  );

		l_return = pack_telegram ( l_telegram );
	}
	else
	{
		l_return = Err ( l_verify.unwrap_err ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_coils ( )
{
	let l_result_1 : u8 = get_expected_byte_count_read_coils ( 8 );

	assert_eq! ( l_result_1, 10 );


	let l_result_2 : u8 = get_expected_byte_count_read_coils ( 16 );

	assert_eq! ( l_result_2, 11 );


	let l_result_3 : u8 = get_expected_byte_count_read_coils ( 7 );

	assert_eq! ( l_result_3, 10 );


	let l_result_4 : u8 = get_expected_byte_count_read_coils ( 19 );

	assert_eq! ( l_result_4, 12 );
}

fn get_expected_byte_count_read_coils ( quantity_of_coils : u16 ) -> u8
{
	let mut l_return : u8 =	MODBUS_HEADER_SIZE + 2;	// +2 für FunctionCode und ByteCount

	if ( quantity_of_coils % 8 ) > 0
	{
		l_return += ( ( quantity_of_coils / 8 ) + 1 ) as u8;
	}
	else
	{
		l_return += ( quantity_of_coils / 8 ) as u8;
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_discrete_inputs ( )
{
	let l_result_1 : u8 = get_expected_byte_count_read_discrete_inputs ( 8 );

	assert_eq! ( l_result_1, 10 );


	let l_result_2 : u8 = get_expected_byte_count_read_discrete_inputs ( 16 );

	assert_eq! ( l_result_2, 11 );


	let l_result_3 : u8 = get_expected_byte_count_read_discrete_inputs ( 7 );

	assert_eq! ( l_result_3, 10 );


	let l_result_4 : u8 = get_expected_byte_count_read_discrete_inputs ( 19 );

	assert_eq! ( l_result_4, 12 );
}

fn get_expected_byte_count_read_discrete_inputs ( quantity_of_inputs : u16 ) -> u8
{
	let mut l_return : u8 =	MODBUS_HEADER_SIZE + 2;	// +2 für FunctionCode und ByteCount

	if ( quantity_of_inputs % 8 ) > 0
	{
		l_return += ( ( quantity_of_inputs / 8 ) + 1 ) as u8;
	}
	else
	{
		l_return += ( quantity_of_inputs / 8 ) as u8;
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_holding_registers ( )
{
	let l_result : u8 =	get_expected_byte_count_read_holding_registers ( 20 );
	
	assert_eq! ( l_result, 49 );
}

fn get_expected_byte_count_read_holding_registers ( quantity_of_registers : u16 ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + ( quantity_of_registers * 2 ) as u8 + 2; // +2 für FunctionCode und ByteCount

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_input_registers ( )
{
	let l_result : u8 =	get_expected_byte_count_read_input_registers ( 20 );
	
	assert_eq! ( l_result, 49 );
}

fn get_expected_byte_count_read_input_registers ( quantity_of_input_registers : u16 ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + ( quantity_of_input_registers * 2 ) as u8 + 2; // +2 für FunctionCode und ByteCount

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_multiple_coils ( )
{
	let l_result : u8 =	get_expected_byte_count_write_multiple_coils ( );
	
	assert_eq! ( l_result, 12 );
}

fn get_expected_byte_count_write_multiple_coils ( ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + 5; // +5 für FunctionCode, 2 Bytes StartingAddress & 2 Bytes QuantityOfOutputs

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_multiple_registers ( )
{
	let l_result : u8 =	get_expected_byte_count_write_multiple_registers ( );
	
	assert_eq! ( l_result, 12 );
}

fn get_expected_byte_count_write_multiple_registers ( ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + 5; // +5 für FunctionCode, 2 Bytes StartingAddress & 2 Bytes QuantityOfRegisters

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_single_coil ( )
{
	let l_result : u8 =	get_expected_byte_count_write_single_coil ( );
	
	assert_eq! ( l_result, 12 );
}

fn get_expected_byte_count_write_single_coil ( ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + 5; // +5 für FunctionCode, 2 Bytes OutputAddress & 2 Bytes OutputValue

	return l_return;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_single_register ( )
{
	let l_result : u8 =	get_expected_byte_count_write_single_register ( );
	
	assert_eq! ( l_result, 12 );
}

fn get_expected_byte_count_write_single_register ( ) -> u8
{
	let l_return : u8 =	MODBUS_HEADER_SIZE + 5; // +5 für FunctionCode, 2 Bytes RegisterAddress & 2 Bytes RegisterValue

	return l_return;
}

//	===============================================================================================

#[test]
fn test_pack_telegram ( )
{
	let l_test_1_data : Option<ModbusTelegram> = None;

	let l_result_1_data : Result<ModbusTelegram, String> = pack_telegram ( l_test_1_data );

	assert_eq! ( l_result_1_data.is_err ( ), true );


	let l_test_vec : Vec<u8> = vec![0;10];
	let l_test_2_data : Option<ModbusTelegram> = ModbusTelegram::new ( 0x0001, 0x01, 0x01, &l_test_vec, 10 );

	let l_result_2_data : Result<ModbusTelegram, String> = pack_telegram ( l_test_2_data );

	assert_eq! ( l_result_2_data.is_ok ( ), true );
}

fn pack_telegram ( telegram : Option<ModbusTelegram> ) -> Result<ModbusTelegram, String>
{
	let l_return : Result<ModbusTelegram, String>;

	if telegram.is_some ( )
	{
		l_return = Ok ( telegram.unwrap ( ) );
	}
	else
	{
		l_return = Err ( "Error while creating telegram.".to_string ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_coils ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_read_coils ( 0x00FF, 0x000A );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x00 );
	assert_eq! ( l_result_1_data[3], 0x0A );
}

fn prepare_payload_read_coils ( starting_address : u16, quantity_of_coils : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];

	append_word_to_bytearray ( &mut l_return, starting_address );
							 	 
	append_word_to_bytearray ( &mut l_return, quantity_of_coils );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_discrete_inputs ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_read_discrete_inputs ( 0x00FF, 0x000A );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x00 );
	assert_eq! ( l_result_1_data[3], 0x0A );
}

fn prepare_payload_read_discrete_inputs ( starting_address : u16, quantity_of_inputs : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];

	append_word_to_bytearray ( &mut l_return, starting_address );
							 	 
	append_word_to_bytearray ( &mut l_return, quantity_of_inputs );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_input_registers ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_read_input_registers ( 0x00FF, 0x000A );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x00 );
	assert_eq! ( l_result_1_data[3], 0x0A );
}

fn prepare_payload_read_input_registers ( starting_address : u16, quantity_of_input_registers : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];

	append_word_to_bytearray ( &mut l_return, starting_address );
							 	 
	append_word_to_bytearray ( &mut l_return, quantity_of_input_registers );

	return l_return;	
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_holding_registers ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_read_holding_registers ( 0x00FF, 0x0001 );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x00 );
	assert_eq! ( l_result_1_data[3], 0x01 );
}

fn prepare_payload_read_holding_registers ( starting_address : u16, quantity_of_registers : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];
	
	append_word_to_bytearray ( &mut l_return, starting_address );
							 	 
	append_word_to_bytearray ( &mut l_return, quantity_of_registers );
	
	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_multiple_coils ( )
{
	let mut l_output_values : Vec<u8> = vec![];

	l_output_values.push ( 0x0F );
	l_output_values.push ( 0xF0 );


	let l_result_1_data : Vec<u8> =	prepare_payload_write_multiple_coils ( 0x0001, 0x0010, &l_output_values );

	assert_eq! ( l_result_1_data.len ( ), 7 );

	assert_eq! ( l_result_1_data[ 0], 0x00 );
	assert_eq! ( l_result_1_data[ 1], 0x01 );
	assert_eq! ( l_result_1_data[ 2], 0x00 );
	assert_eq! ( l_result_1_data[ 3], 0x10 );
	assert_eq! ( l_result_1_data[ 4], 0x02 );
	assert_eq! ( l_result_1_data[ 5], 0x0F );
	assert_eq! ( l_result_1_data[ 6], 0xF0 );
}

fn prepare_payload_write_multiple_coils ( starting_address : u16, quantity_of_outputs : u16, output_values : &Vec<u8> ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];

	append_word_to_bytearray ( &mut l_return, starting_address );

	append_word_to_bytearray ( &mut l_return, quantity_of_outputs );

	append_byte_to_bytearray ( &mut l_return, output_values.len ( ) as u8 );

	append_bytearray_to_bytearray ( &mut l_return, &output_values );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_multiple_registers ( )
{
	let mut l_register_values : Vec<u16> = vec![];

	l_register_values.push ( 0x00FF );
	l_register_values.push ( 0xF00F );
	l_register_values.push ( 0x010A );
	l_register_values.push ( 0xABCD );


	let l_result_1_data : Vec<u8> =	prepare_payload_write_multiple_registers ( 0x0100, &l_register_values );

	assert_eq! ( l_result_1_data.len ( ), 13 );

	assert_eq! ( l_result_1_data[ 0], 0x01 );
	assert_eq! ( l_result_1_data[ 1], 0x00 );
	assert_eq! ( l_result_1_data[ 2], 0x00 );
	assert_eq! ( l_result_1_data[ 3], 0x04 );
	assert_eq! ( l_result_1_data[ 4], 0x08 );
	assert_eq! ( l_result_1_data[ 5], 0x00 );
	assert_eq! ( l_result_1_data[ 6], 0xFF );
	assert_eq! ( l_result_1_data[ 7], 0xF0 );
	assert_eq! ( l_result_1_data[ 8], 0x0F );
	assert_eq! ( l_result_1_data[ 9], 0x01 );
	assert_eq! ( l_result_1_data[10], 0x0A );
	assert_eq! ( l_result_1_data[11], 0xAB );
	assert_eq! ( l_result_1_data[12], 0xCD );
}

fn prepare_payload_write_multiple_registers ( starting_address : u16, register_values : &Vec<u16> ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];
	
	append_word_to_bytearray ( &mut l_return, starting_address );

	append_word_to_bytearray ( &mut l_return, register_values.len ( ) as u16 );

	let l_register_values_bytes : Vec<u8> =	transform_words_to_bytes ( &register_values );

	append_byte_to_bytearray ( &mut l_return, l_register_values_bytes.len ( ) as u8 );

	append_bytearray_to_bytearray ( &mut l_return, &l_register_values_bytes );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_single_coil ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_write_single_coil ( 0x00FF, 0x0000 );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x00 );
	assert_eq! ( l_result_1_data[3], 0x00 );


	let l_result_2_data : Vec<u8> =	prepare_payload_write_single_coil ( 0x00FF, 0xFF00 );

	assert_eq! ( l_result_2_data.len ( ), 4 );

	assert_eq! ( l_result_2_data[0], 0x00 );
	assert_eq! ( l_result_2_data[1], 0xFF );
	assert_eq! ( l_result_2_data[2], 0xFF );
	assert_eq! ( l_result_2_data[3], 0x00 );
}

fn prepare_payload_write_single_coil ( output_address : u16, output_value : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];
	
	append_word_to_bytearray ( &mut l_return, output_address );

	append_word_to_bytearray ( &mut l_return, output_value );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_single_register ( )
{
	let l_result_1_data : Vec<u8> =	prepare_payload_write_single_register ( 0x00FF, 0x010A );

	assert_eq! ( l_result_1_data.len ( ), 4 );

	assert_eq! ( l_result_1_data[0], 0x00 );
	assert_eq! ( l_result_1_data[1], 0xFF );
	assert_eq! ( l_result_1_data[2], 0x01 );
	assert_eq! ( l_result_1_data[3], 0x0A );
}

fn prepare_payload_write_single_register ( register_address : u16, register_value : u16 ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];
	
	append_word_to_bytearray ( &mut l_return, register_address );

	append_word_to_bytearray ( &mut l_return, register_value );

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_coils ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x03 );
	l_test_1_data.push ( 0xCD );
	l_test_1_data.push ( 0x6B );
	l_test_1_data.push ( 0x05 );	
	
	
	let l_result_1_data : Vec<bool> = prepare_response_read_coils ( &l_test_1_data, 0x0013 );

	assert_eq! ( l_result_1_data.len ( ), 19 );
	
	assert_eq! ( l_result_1_data[0], true );
	assert_eq! ( l_result_1_data[1], false );
	assert_eq! ( l_result_1_data[2], true );
	assert_eq! ( l_result_1_data[3], true );
	assert_eq! ( l_result_1_data[4], false );
	assert_eq! ( l_result_1_data[5], false );
	assert_eq! ( l_result_1_data[6], true );
	assert_eq! ( l_result_1_data[7], true );
	assert_eq! ( l_result_1_data[8], true );
	assert_eq! ( l_result_1_data[9], true );
	assert_eq! ( l_result_1_data[10], false );
	assert_eq! ( l_result_1_data[11], true );
	assert_eq! ( l_result_1_data[12], false );
	assert_eq! ( l_result_1_data[13], true );
	assert_eq! ( l_result_1_data[14], true );
	assert_eq! ( l_result_1_data[15], false );
	assert_eq! ( l_result_1_data[16], true );
	assert_eq! ( l_result_1_data[17], false );
	assert_eq! ( l_result_1_data[18], true );
}

pub fn prepare_response_read_coils ( payload : &Vec<u8>, coil_count : u16 ) -> Vec<bool>
{
	let mut l_return : Vec<bool> = vec![];

	if payload.len ( ) > 3
	{
		let l_byte_count_option : Option<u8> = extract_byte_from_bytearray ( &payload, 0 );

		if l_byte_count_option.is_some ( )
		{
			let l_byte_count : u8 =	l_byte_count_option.unwrap ( );
	
			let l_coil_values_option : Option<Vec<u8>> = extract_bytes_from_bytearray ( &payload, 1, l_byte_count );

			if l_coil_values_option.is_some ( )
			{
				let l_coil_bytes : Vec<u8> = l_coil_values_option.unwrap ( );

				let mut l_byte_index : usize = 0;
				let mut l_bit : u8 = 0;

				for _ in 0..coil_count
				{
					l_return.push ( l_coil_bytes[ l_byte_index ] & ( 1 << l_bit ) != 0 );

					if l_bit == 7
					{
						l_byte_index += 1;
						l_bit = 0;
					}
					else
					{
						l_bit += 1;
					}
				}
			}
		}
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_discrete_inputs ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x03 );
	l_test_1_data.push ( 0xAC );
	l_test_1_data.push ( 0xDB );
	l_test_1_data.push ( 0x35 );	
	
	
	let l_result_1_data : Vec<bool> = prepare_response_read_discrete_inputs ( &l_test_1_data, 0x0016 );

	assert_eq! ( l_result_1_data.len ( ), 22 );
	
	assert_eq! ( l_result_1_data[0], false );
	assert_eq! ( l_result_1_data[1], false );
	assert_eq! ( l_result_1_data[2], true );
	assert_eq! ( l_result_1_data[3], true );
	assert_eq! ( l_result_1_data[4], false );
	assert_eq! ( l_result_1_data[5], true );
	assert_eq! ( l_result_1_data[6], false );
	assert_eq! ( l_result_1_data[7], true );
	assert_eq! ( l_result_1_data[8], true );
	assert_eq! ( l_result_1_data[9], true );
	assert_eq! ( l_result_1_data[10], false );
	assert_eq! ( l_result_1_data[11], true );
	assert_eq! ( l_result_1_data[12], true );
	assert_eq! ( l_result_1_data[13], false );
	assert_eq! ( l_result_1_data[14], true );
	assert_eq! ( l_result_1_data[15], true );
	assert_eq! ( l_result_1_data[16], true );
	assert_eq! ( l_result_1_data[17], false );
	assert_eq! ( l_result_1_data[18], true );
	assert_eq! ( l_result_1_data[19], false );
	assert_eq! ( l_result_1_data[20], true );
	assert_eq! ( l_result_1_data[21], true );
}

pub fn prepare_response_read_discrete_inputs ( payload : &Vec<u8>, input_count : u16 ) -> Vec<bool>
{
	let mut l_return : Vec<bool> = vec![];

	if payload.len ( ) > 3
	{
		let l_byte_count_option : Option<u8> = extract_byte_from_bytearray ( &payload, 0 );

		if l_byte_count_option.is_some ( )
		{
			let l_byte_count : u8 = l_byte_count_option.unwrap ( );
	
			let l_input_values_option : Option<Vec<u8>> = extract_bytes_from_bytearray ( &payload, 1, l_byte_count );

			if l_input_values_option.is_some ( )
			{
				let l_input_bytes : Vec<u8> = l_input_values_option.unwrap ( );

				let mut l_byte_index : usize = 0;
				let mut l_bit : u8 = 0;

				for _ in 0..input_count
				{
					l_return.push ( l_input_bytes[ l_byte_index ] & ( 1 << l_bit ) != 0 );

					if l_bit == 7
					{
						l_byte_index += 1;
						l_bit = 0;
					}
					else
					{
						l_bit += 1;
					}
				}
			}
		}
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_holding_registers ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x06 );
	l_test_1_data.push ( 0xF0 );
	l_test_1_data.push ( 0x0F );
	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0xFF );
	l_test_1_data.push ( 0xFF );
	l_test_1_data.push ( 0x00 );

	let l_result_1_data : Vec<u16> = prepare_response_read_holding_registers ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 3 );

	assert_eq! ( l_result_1_data[0], 0xF00F );
	assert_eq! ( l_result_1_data[1], 0x00FF );
	assert_eq! ( l_result_1_data[2], 0xFF00 );
}

pub fn prepare_response_read_holding_registers ( payload : &Vec<u8> ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	if payload.len ( ) > 3
	{
		let l_byte_count_option : Option<u8> = extract_byte_from_bytearray ( &payload, 0 );

		if l_byte_count_option.is_some ( )
		{
			let l_byte_count : u8 = l_byte_count_option.unwrap ( );
	
			let l_register_values_option : Option<Vec<u8>> = extract_bytes_from_bytearray ( &payload, 1, l_byte_count );

			if l_register_values_option.is_some ( )
			{
				let l_words : u8 = ( l_byte_count / 2 ) as u8;

				let l_transformed_words : Vec<u16> = transform_bytes_to_words ( &l_register_values_option.unwrap ( ), 0, l_words );

				l_return = l_transformed_words;
			}
		}		
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_input_registers ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x06 );
	l_test_1_data.push ( 0xF0 );
	l_test_1_data.push ( 0x0F );
	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0xFF );
	l_test_1_data.push ( 0xFF );
	l_test_1_data.push ( 0x00 );

	let l_result_1_data : Vec<u16> = prepare_response_read_input_registers ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 3 );

	assert_eq! ( l_result_1_data[0], 0xF00F );
	assert_eq! ( l_result_1_data[1], 0x00FF );
	assert_eq! ( l_result_1_data[2], 0xFF00 );
}

pub fn prepare_response_read_input_registers ( payload : &Vec<u8> ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	if payload.len ( ) > 3
	{
		let l_byte_count_option : Option<u8> = extract_byte_from_bytearray ( &payload, 0 );

		if l_byte_count_option.is_some ( )
		{
			let l_byte_count : u8 = l_byte_count_option.unwrap ( );
	
			let l_register_values_option : Option<Vec<u8>> = extract_bytes_from_bytearray ( &payload, 1, l_byte_count );

			if l_register_values_option.is_some ( )
			{
				let l_words : u8 = ( l_byte_count / 2 ) as u8;

				let l_transformed_words : Vec<u16> = transform_bytes_to_words ( &l_register_values_option.unwrap ( ), 0, l_words );

				l_return = l_transformed_words;
			}
		}		
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_multiple_coils ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x01 );
	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x03 );

	let l_result_1_data : Vec<u16> = prepare_response_write_multiple_coils ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 2 );

	assert_eq! ( l_result_1_data[0], 0x0001 );
	assert_eq! ( l_result_1_data[1], 0x0003 );
}

pub fn prepare_response_write_multiple_coils ( payload : &Vec<u8> ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	if payload.len ( ) == 4
	{
		l_return = transform_bytes_to_words ( &payload, 0, 2 );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_multiple_registers ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x01 );
	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0xFF );

	let l_result_1_data : Vec<u16> = prepare_response_write_multiple_registers ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 2 );

	assert_eq! ( l_result_1_data[0], 0x0001 );
	assert_eq! ( l_result_1_data[1], 0x00FF );
}

pub fn prepare_response_write_multiple_registers ( payload : &Vec<u8> ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	if payload.len ( ) == 4
	{
		let l_option_address : Option<u16> = extract_word_from_bytearray ( payload, 0 );

		let l_option_quantity : Option<u16> = extract_word_from_bytearray ( payload, 2 );

		if l_option_address.is_some ( ) &&
		   l_option_quantity.is_some ( )
		{
			l_return.push ( l_option_address.unwrap ( ) );
			l_return.push ( l_option_quantity.unwrap ( ) );
		}
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_single_coil ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x01 );
	l_test_1_data.push ( 0xFF );
	l_test_1_data.push ( 0x00 );

	let l_result_1_data : Vec<bool> = prepare_response_write_single_coil ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 1 );

	assert_eq! ( l_result_1_data[0], true );


	let mut l_test_2_data : Vec<u8> = vec![];

	l_test_2_data.push ( 0x00 );
	l_test_2_data.push ( 0x01 );
	l_test_2_data.push ( 0x00 );
	l_test_2_data.push ( 0x00 );

	let l_result_2_data : Vec<bool> = prepare_response_write_single_coil ( &l_test_2_data );

	assert_eq! ( l_result_2_data.len ( ), 1 );

	assert_eq! ( l_result_2_data[0], false );


	let mut l_test_3_data : Vec<u8> = vec![];

	l_test_3_data.push ( 0x00 );
	l_test_3_data.push ( 0x01 );
	l_test_3_data.push ( 0x0F );
	l_test_3_data.push ( 0xF0 );

	let l_result_3_data : Vec<bool> = prepare_response_write_single_coil ( &l_test_3_data );

	assert_eq! ( l_result_3_data.len ( ), 1 );

	assert_eq! ( l_result_3_data[0], false );
}

pub fn prepare_response_write_single_coil ( payload : &Vec<u8> ) -> Vec<bool>
{
	let mut l_return : Vec<bool> = vec![];

	if payload.len ( ) == 4
	{
		let l_option_word : Option<u16> = extract_word_from_bytearray ( payload, 2 );
		
		if l_option_word.is_some ( )
		{
			let l_word : u16 = l_option_word.unwrap ( );

			if l_word == 0xFF00
			{
				l_return.push ( true );
			}
			else
			{
				l_return.push ( false );
			}
		}
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_single_register ( )
{
	let mut l_test_1_data : Vec<u8> = vec![];

	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x01 );
	l_test_1_data.push ( 0x00 );
	l_test_1_data.push ( 0x03 );

	let l_result_1_data : Vec<u16> = prepare_response_write_single_register ( &l_test_1_data );

	assert_eq! ( l_result_1_data.len ( ), 2 );

	assert_eq! ( l_result_1_data[0], 0x0001 );
	assert_eq! ( l_result_1_data[1], 0x0003 );
}

pub fn prepare_response_write_single_register ( payload : &Vec<u8> ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	if payload.len ( ) == 4
	{
		l_return = transform_bytes_to_words ( &payload, 0, 2 );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_is_start_and_quantity_ok ( )
{	
	assert_eq! ( is_start_and_quantity_ok ( 0x0000, 0x00FF ), true );
	assert_eq! ( is_start_and_quantity_ok ( 0x0000, 0xFFFF ), true );
	assert_eq! ( is_start_and_quantity_ok ( 0x0100, 0x00FF ), true );
	assert_eq! ( is_start_and_quantity_ok ( 0xFFFE, 0x0001 ), true );
	assert_eq! ( is_start_and_quantity_ok ( 0x0100, 0xFFFF ), false );
}

fn is_start_and_quantity_ok ( start : u16, quantity : u16 ) -> bool
{
	let l_return : bool;

	if ( start as u32 + quantity as u32 ) <= 0x0000FFFF
	{
		l_return = true;
	}
	else
	{
		l_return = false;
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_is_value_in_range ( )
{
	assert_eq! ( is_value_in_range ( 0x0002, 0x0000, 0x000A ), true );
	assert_eq! ( is_value_in_range ( 0x0000, 0x0000, 0x000A ), true );
	assert_eq! ( is_value_in_range ( 0x000A, 0x0000, 0x000A ), true );
	assert_eq! ( is_value_in_range ( 0x000B, 0x0000, 0x000A ), false );
	assert_eq! ( is_value_in_range ( 0x0007, 0x0008, 0x000A ), false );
}

fn is_value_in_range ( value : u16, min_value : u16, max_value : u16 ) -> bool
{
	let l_return : bool;

	if value >= min_value && value <= max_value
	{
		l_return = true;
	}
	else
	{
		l_return = false;
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_coils ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_read_coils ( 0x0000, 0x0001 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_read_coils ( 0x0000, 0x07D0 );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_read_coils ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_read_coils ( 0x0000, 0x07D1 );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_read_coils ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );
}

fn verify_parameter_read_coils ( starting_address : u16, quantity_of_coils : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool =	is_start_and_quantity_ok ( starting_address, quantity_of_coils );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_coils is over 65535.".to_string ( ) );
	}


	let l_quantity_good : bool;

	if l_address_good
	{
		//	Die Anzahl der zu lesenden Register muss zwischen 1 und 2000 liegen
		if is_value_in_range ( quantity_of_coils, 0x0001, 0x07D0 )
		{
			l_quantity_good = true ;
		}
		else
		{
			l_quantity_good = false ;

			if quantity_of_coils == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_coils - value to low, must be over 1.".to_string ( ) );
			}

			if quantity_of_coils > 0x07D0
			{
				l_return = Err ( "Error at parameter quantity_of_coils - value to high, must be lower or equal 2000.".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}

	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_discrete_inputs ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_read_discrete_inputs ( 0x0000, 0x0001 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_read_discrete_inputs ( 0x0000, 0x07D0 );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_read_discrete_inputs ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_read_discrete_inputs ( 0x0000, 0x07D1 );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_read_discrete_inputs ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );
}

fn verify_parameter_read_discrete_inputs ( starting_address : u16, quantity_of_inputs : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool =	is_start_and_quantity_ok ( starting_address, quantity_of_inputs );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_inputs is over 65535.".to_string ( ) );
	}


	let l_quantity_good : bool;

	if l_address_good
	{
		//	Die Anzahl der zu lesenden Register muss zwischen 1 und 2000 liegen
		if is_value_in_range ( quantity_of_inputs, 0x0001, 0x07D0 )
		{
			l_quantity_good = true ;
		}
		else
		{
			l_quantity_good = false ;

			if quantity_of_inputs == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_inputs - value to low, must be over 1.".to_string ( ) );
			}

			if quantity_of_inputs > 0x07D0
			{
				l_return = Err ( "Error at parameter quantity_of_inputs - value to high, must be lower or equal 2000.".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}

	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_holding_registers ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_read_holding_registers ( 0x0000, 0x0001 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_read_holding_registers ( 0x0000, 0x007D );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_read_holding_registers ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_read_holding_registers ( 0x0000, 0x007E );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_read_holding_registers ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );
}

fn verify_parameter_read_holding_registers ( starting_address : u16, quantity_of_registers : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool = is_start_and_quantity_ok ( starting_address, quantity_of_registers );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_registers is over 65535.".to_string ( ) );
	}


	let l_quantity_good : bool;

	if l_address_good
	{
		//	Die Anzahl der zu lesenden Register muss zwischen 1 und 125 liegen
		if is_value_in_range ( quantity_of_registers, 0x0001, 0x007D )
		{
			l_quantity_good = true ;
		}
		else
		{
			l_quantity_good = false ;

			if quantity_of_registers == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_registers - value to low, must be over 1.".to_string ( ) );
			}

			if quantity_of_registers > 0x007D
			{
				l_return = Err ( "Error at parameter quantity_of_registers - value to high, must be lower or equal 125.".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}

	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_input_registers ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_read_input_registers ( 0x0000, 0x0001 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_read_input_registers ( 0x0000, 0x007D );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_read_input_registers ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_read_input_registers ( 0x0000, 0x007E );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_read_input_registers ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );
}

fn verify_parameter_read_input_registers ( starting_address : u16, quantity_of_input_registers : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool =	is_start_and_quantity_ok ( starting_address, quantity_of_input_registers );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_input_registers is over 65535.".to_string ( ) );
	}


	let l_quantity_good : bool;

	if l_address_good
	{
		//	Die Anzahl der zu lesenden Register muss zwischen 1 und 125 liegen
		if is_value_in_range ( quantity_of_input_registers, 0x0001, 0x007D )
		{
			l_quantity_good = true ;
		}
		else
		{
			l_quantity_good = false ;

			if quantity_of_input_registers == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_input_registers - value to low, must be over 1.".to_string ( ) );
			}

			if quantity_of_input_registers > 0x007D
			{
				l_return = Err ( "Error at parameter quantity_of_input_registers - value to high, must be lower or equal 125.".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}


	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_multiple_coils ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_write_multiple_coils ( 0x0000, 0x0001 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_write_multiple_coils ( 0x0000, 0x07B0 );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_write_multiple_coils ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_write_multiple_coils ( 0x0000, 0x07B1 );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_write_multiple_coils ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );	
}

fn verify_parameter_write_multiple_coils ( starting_address : u16, quantity_of_output_values : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool = is_start_and_quantity_ok ( starting_address, quantity_of_output_values );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_output_values is over 65535.".to_string ( ) );
	}


	let l_quantity_good : bool;

	if l_address_good
	{
		//	Die Anzahl der zu lesenden Werte muss zwischen 1 und 1968 liegen
		if is_value_in_range ( quantity_of_output_values, 0x0001, 0x07B0 )
		{
			l_quantity_good = true ;
		}
		else
		{
			l_quantity_good = false ;

			if quantity_of_output_values == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_output_values - value to low, must be over 1.".to_string ( ) );
			}

			if quantity_of_output_values > 0x07B0
			{
				l_return = Err ( "Error at parameter quantity_of_output_values - value to high, must be lower or equal 1968.".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}


	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_multiple_registers ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_write_multiple_registers ( 0x0000, 0x000F );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_write_multiple_registers ( 0x0000, 0x007B );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_write_multiple_registers ( 0x0000, 0x0000 );

	assert_eq! ( l_result_3.is_err ( ), true );


	let l_result_4 : Result<bool, String> =	verify_parameter_write_multiple_registers ( 0x0000, 0x007C );

	assert_eq! ( l_result_4.is_err ( ), true );


	let l_result_5 : Result<bool, String> =	verify_parameter_write_multiple_registers ( 0xFFFE, 0x000F );

	assert_eq! ( l_result_5.is_err ( ), true );
}

fn verify_parameter_write_multiple_registers ( starting_address : u16, quantity_of_registers : u16 ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Ok ( false );

	let l_address_good : bool =	is_start_and_quantity_ok ( starting_address, quantity_of_registers );

	if l_address_good
	{		
	}
	else
	{
		l_return = Err ( "Error - range or starting_address and quantity_of_registers is over 65535.".to_string ( ) );
	}

	
	let l_quantity_good : bool;

	if l_address_good
	{
		if is_value_in_range ( quantity_of_registers, 0x0001, 0x007B )
		{
			l_quantity_good = true;
		}
		else
		{
			l_quantity_good = false;

			if quantity_of_registers == 0x0000
			{
				l_return = Err ( "Error at parameter quantity_of_registers - value to low, must be over 1".to_string ( ) );
			}

			if quantity_of_registers > 0x007B
			{
				l_return = Err ( "Error at parameter quantity_of_registers - value to high, must be lower or equal 123".to_string ( ) );
			}
		}
	}
	else
	{
		l_quantity_good = false;
	}


	if l_address_good && l_quantity_good
	{
		l_return = Ok ( true );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_single_coil ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_write_single_coil ( 0x0000 );

	assert_eq! ( l_result_1.is_ok ( ), true );


	let l_result_2 : Result<bool, String> =	verify_parameter_write_single_coil ( 0xFF00 );

	assert_eq! ( l_result_2.is_ok ( ), true );


	let l_result_3 : Result<bool, String> =	verify_parameter_write_single_coil ( 0x0F0F );

	assert_eq! ( l_result_3.is_err ( ), true );
}

fn verify_parameter_write_single_coil ( output_value : u16 ) -> Result<bool, String>
{
	let l_return : Result<bool, String>;

	if output_value == 0x0000 || output_value == 0xFF00
	{
		l_return = Ok ( true );
	}
	else
	{
		l_return = Err ( "Error at parameter output_value - valid values are only 0 [0x0000] or 65280 [0xFF00].".to_string ( ) );
	}

	return l_return;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_single_register ( )
{
	let l_result_1 : Result<bool, String> =	verify_parameter_write_single_register ( );

	assert_eq! ( l_result_1.is_ok ( ), true );
}

fn verify_parameter_write_single_register ( ) -> Result<bool, String>
{
	return Ok ( true );
}
