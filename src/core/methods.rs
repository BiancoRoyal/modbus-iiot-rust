use core::consts::*;
use core::datatransformation::*;
use core::modbustelegram::ModbusTelegram;

//	===============================================================================================

#[test]
fn test_create_request_read_coils() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let quantity_of_coils: u16 = 0x000A;

    let result: Result<ModbusTelegram, String> = create_request_read_coils(
        transaction_identifier,
        unit_identifier,
        starting_address,
        quantity_of_coils,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_READ_COILS);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);

    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_READ_COILS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_coils
    assert_eq!(bytes[11], 0x0A); //	quantity_of_coils
}

pub fn create_request_read_coils(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    quantity_of_coils: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_read_coils(starting_address, quantity_of_coils);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> = prepare_payload_read_coils(starting_address, quantity_of_coils);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_READ_COILS,
            &payload,
            get_expected_byte_count_read_coils(quantity_of_coils),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_read_discrete_inputs() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let quantity_of_inputs: u16 = 0x000A;

    let result: Result<ModbusTelegram, String> = create_request_read_discrete_inputs(
        transaction_identifier,
        unit_identifier,
        starting_address,
        quantity_of_inputs,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_READ_DISCRETE_INPUTS);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_READ_DISCRETE_INPUTS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_inputs
    assert_eq!(bytes[11], 0x0A); //	quantity_of_inputs
}

pub fn create_request_read_discrete_inputs(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    quantity_of_inputs: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_read_discrete_inputs(starting_address, quantity_of_inputs);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> =
            prepare_payload_read_discrete_inputs(starting_address, quantity_of_inputs);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_READ_DISCRETE_INPUTS,
            &payload,
            get_expected_byte_count_read_discrete_inputs(quantity_of_inputs),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_read_holding_registers() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let quantity_of_registers: u16 = 0x000A;

    let result: Result<ModbusTelegram, String> = create_request_read_holding_registers(
        transaction_identifier,
        unit_identifier,
        starting_address,
        quantity_of_registers,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_READ_HOLDING_REGISTERS);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_READ_HOLDING_REGISTERS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_registers
    assert_eq!(bytes[11], 0x0A); //	quantity_of_registers
}

pub fn create_request_read_holding_registers(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    quantity_of_registers: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_read_holding_registers(starting_address, quantity_of_registers);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> =
            prepare_payload_read_holding_registers(starting_address, quantity_of_registers);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_READ_HOLDING_REGISTERS,
            &payload,
            get_expected_byte_count_read_holding_registers(quantity_of_registers),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_read_input_registers() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let quantity_of_input_registers: u16 = 0x000A;

    let result: Result<ModbusTelegram, String> = create_request_read_input_registers(
        transaction_identifier,
        unit_identifier,
        starting_address,
        quantity_of_input_registers,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_READ_INPUT_REGISTERS);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_READ_INPUT_REGISTERS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_input_registers
    assert_eq!(bytes[11], 0x0A); //	quantity_of_input_registers
}

pub fn create_request_read_input_registers(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    quantity_of_input_registers: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_read_input_registers(starting_address, quantity_of_input_registers);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> =
            prepare_payload_read_input_registers(starting_address, quantity_of_input_registers);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_READ_INPUT_REGISTERS,
            &payload,
            get_expected_byte_count_read_input_registers(quantity_of_input_registers),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_write_multiple_coils() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let quantity_of_outputs: u16 = 18;
    let output_values: Vec<u8> = vec![0xFF, 0xF0, 0x02];

    let result: Result<ModbusTelegram, String> = create_request_write_multiple_coils(
        transaction_identifier,
        unit_identifier,
        starting_address,
        quantity_of_outputs,
        output_values,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some(),);
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_WRITE_MULTIPLE_COILS);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 16);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x0A); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_WRITE_MULTIPLE_COILS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_outputs
    assert_eq!(bytes[11], 0x12); //	quantity_of_outputs
    assert_eq!(bytes[12], 0x03); //	byte_count
    assert_eq!(bytes[13], 0xFF); //	output_value
    assert_eq!(bytes[14], 0xF0); //	output_value
    assert_eq!(bytes[15], 0x02); //	output_value
}

pub fn create_request_write_multiple_coils(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    quantity_of_outputs: u16,
    output_values: Vec<u8>,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_write_multiple_coils(starting_address, quantity_of_outputs);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> = prepare_payload_write_multiple_coils(
            starting_address,
            quantity_of_outputs,
            &output_values,
        );

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_WRITE_MULTIPLE_COILS,
            &payload,
            get_expected_byte_count_write_multiple_coils(),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_write_multiple_registers() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let starting_address: u16 = 0x00FF;
    let register_values: Vec<u16> = vec![0x00FF, 0xF00F, 0x010A, 0xABCD];

    let result: Result<ModbusTelegram, String> = create_request_write_multiple_registers(
        transaction_identifier,
        unit_identifier,
        starting_address,
        register_values,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(
        function_code.unwrap(),
        FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS
    );

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 21);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x0F); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS);
    assert_eq!(bytes[8], 0x00); //	starting_address
    assert_eq!(bytes[9], 0xFF); //	starting_address
    assert_eq!(bytes[10], 0x00); //	quantity_of_registers
    assert_eq!(bytes[11], 0x04); //	quantity_of_registers
    assert_eq!(bytes[12], 0x08); //	byte_count
    assert_eq!(bytes[13], 0x00); //	register_value
    assert_eq!(bytes[14], 0xFF); //	register_value
    assert_eq!(bytes[15], 0xF0); //	register_value
    assert_eq!(bytes[16], 0x0F); //	register_value
    assert_eq!(bytes[17], 0x01); //	register_value
    assert_eq!(bytes[18], 0x0A); //	register_value
    assert_eq!(bytes[19], 0xAB); //	register_value
    assert_eq!(bytes[20], 0xCD); //	register_value
}

pub fn create_request_write_multiple_registers(
    transaction_identifier: u16,
    unit_identifier: u8,
    starting_address: u16,
    register_values: Vec<u16>,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let quantity_of_registers: u16 = register_values.len() as u16;
    let parameter_verification: Result<bool, String> =
        verify_parameter_write_multiple_registers(starting_address, quantity_of_registers);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> =
            prepare_payload_write_multiple_registers(starting_address, &register_values);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_WRITE_MULTIPLE_REGISTERS,
            &payload,
            get_expected_byte_count_write_multiple_registers(),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_write_single_coil() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let output_address: u16 = 0x00FF;
    let output_value: u16 = 0xFF00;

    let result: Result<ModbusTelegram, String> = create_request_write_single_coil(
        transaction_identifier,
        unit_identifier,
        output_address,
        output_value,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_WRITE_SINGLE_COIL);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some());

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_WRITE_SINGLE_COIL);
    assert_eq!(bytes[8], 0x00); //	output_address
    assert_eq!(bytes[9], 0xFF); //	output_address
    assert_eq!(bytes[10], 0xFF); //	output_value
    assert_eq!(bytes[11], 0x00); //	output_value
}

pub fn create_request_write_single_coil(
    transaction_identifier: u16,
    unit_identifier: u8,
    output_address: u16,
    output_value: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> =
        verify_parameter_write_single_coil(output_value);

    if parameter_verification.is_ok() {
        let payload: Vec<u8> = prepare_payload_write_single_coil(output_address, output_value);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_WRITE_SINGLE_COIL,
            &payload,
            get_expected_byte_count_write_single_coil(),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_create_request_write_single_register() {
    let transaction_identifier: u16 = 0x00A0;
    let unit_identifier: u8 = 0x01;
    let register_address: u16 = 0x00FF;
    let register_value: u16 = 0xF0F0;

    let result: Result<ModbusTelegram, String> = create_request_write_single_register(
        transaction_identifier,
        unit_identifier,
        register_address,
        register_value,
    );
    assert!(result.is_ok());

    let telegram: ModbusTelegram = result.unwrap();
    let function_code: Option<u8> = telegram.get_function_code();
    assert!(function_code.is_some());
    assert_eq!(function_code.unwrap(), FUNCTION_CODE_WRITE_SINGLE_REGISTER);

    let telegram_bytes: Option<Vec<u8>> = telegram.get_bytes();
    assert!(telegram_bytes.is_some(), true);

    let bytes: Vec<u8> = telegram_bytes.unwrap();
    assert_eq!(bytes.len(), 12);
    assert_eq!(bytes[0], 0x00); //	transaction_identifier
    assert_eq!(bytes[1], 0xA0); //	transaction_identifier
    assert_eq!(bytes[2], 0x00); //	protocol_identifier
    assert_eq!(bytes[3], 0x00); //	protocol_identifier
    assert_eq!(bytes[4], 0x00); //	length of all following bytes
    assert_eq!(bytes[5], 0x06); //	length of all following bytes
    assert_eq!(bytes[6], unit_identifier);
    assert_eq!(bytes[7], FUNCTION_CODE_WRITE_SINGLE_REGISTER);
    assert_eq!(bytes[8], 0x00); //	register_address
    assert_eq!(bytes[9], 0xFF); //	register_address
    assert_eq!(bytes[10], 0xF0); //	register_value
    assert_eq!(bytes[11], 0xF0); //	register_value
}

pub fn create_request_write_single_register(
    transaction_identifier: u16,
    unit_identifier: u8,
    register_address: u16,
    register_value: u16,
) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    let parameter_verification: Result<bool, String> = verify_parameter_write_single_register();

    if parameter_verification.is_ok() {
        let payload: Vec<u8> =
            prepare_payload_write_single_register(register_address, register_value);

        let telegram: Option<ModbusTelegram> = ModbusTelegram::new(
            transaction_identifier,
            unit_identifier,
            FUNCTION_CODE_WRITE_SINGLE_REGISTER,
            &payload,
            get_expected_byte_count_write_single_register(),
        );

        reply = pack_telegram(telegram);
    } else {
        reply = Err(parameter_verification.unwrap_err());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_coils() {
    let result_1: u8 = get_expected_byte_count_read_coils(8);
    assert_eq!(result_1, 10);

    let result_2: u8 = get_expected_byte_count_read_coils(16);
    assert_eq!(result_2, 11);

    let result_3: u8 = get_expected_byte_count_read_coils(7);
    assert_eq!(result_3, 10);

    let result_4: u8 = get_expected_byte_count_read_coils(19);
    assert_eq!(result_4, 12);
}

fn get_expected_byte_count_read_coils(quantity_of_coils: u16) -> u8 {
    let mut reply: u8 = MODBUS_HEADER_SIZE + 2; // +2 for FunctionCode and ByteCount

    if (quantity_of_coils % 8) > 0 {
        reply += ((quantity_of_coils / 8) + 1) as u8;
    } else {
        reply += (quantity_of_coils / 8) as u8;
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_discrete_inputs() {
    let result_1: u8 = get_expected_byte_count_read_discrete_inputs(8);
    assert_eq!(result_1, 10);

    let result_2: u8 = get_expected_byte_count_read_discrete_inputs(16);
    assert_eq!(result_2, 11);

    let result_3: u8 = get_expected_byte_count_read_discrete_inputs(7);
    assert_eq!(result_3, 10);

    let result_4: u8 = get_expected_byte_count_read_discrete_inputs(19);
    assert_eq!(result_4, 12);
}

fn get_expected_byte_count_read_discrete_inputs(quantity_of_inputs: u16) -> u8 {
    let mut reply: u8 = MODBUS_HEADER_SIZE + 2; // +2 für FunctionCode und ByteCount

    if (quantity_of_inputs % 8) > 0 {
        reply += ((quantity_of_inputs / 8) + 1) as u8;
    } else {
        reply += (quantity_of_inputs / 8) as u8;
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_holding_registers() {
    let result: u8 = get_expected_byte_count_read_holding_registers(20);
    assert_eq!(result, 49);
}

fn get_expected_byte_count_read_holding_registers(quantity_of_registers: u16) -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + (quantity_of_registers * 2) as u8 + 2; // +2 for FunctionCode and ByteCount

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_read_input_registers() {
    let result: u8 = get_expected_byte_count_read_input_registers(20);
    assert_eq!(result, 49);
}

fn get_expected_byte_count_read_input_registers(quantity_of_input_registers: u16) -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + (quantity_of_input_registers * 2) as u8 + 2; // +2 for FunctionCode and ByteCount

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_multiple_coils() {
    let result: u8 = get_expected_byte_count_write_multiple_coils();
    assert_eq!(result, 12);
}

fn get_expected_byte_count_write_multiple_coils() -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + 5; // +5 for FunctionCode, 2 Bytes StartingAddress and 2 Bytes QuantityOfOutputs

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_multiple_registers() {
    let result: u8 = get_expected_byte_count_write_multiple_registers();
    assert_eq!(result, 12);
}

fn get_expected_byte_count_write_multiple_registers() -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + 5; // +5 for FunctionCode, 2 Bytes StartingAddress and 2 Bytes QuantityOfRegisters

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_single_coil() {
    let result: u8 = get_expected_byte_count_write_single_coil();
    assert_eq!(result, 12);
}

fn get_expected_byte_count_write_single_coil() -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + 5; // +5 for FunctionCode, 2 Bytes OutputAddress and 2 Bytes OutputValue

    return reply;
}

//	===============================================================================================

#[test]
fn test_get_expected_byte_count_write_single_register() {
    let result: u8 = get_expected_byte_count_write_single_register();
    assert_eq!(result, 12);
}

fn get_expected_byte_count_write_single_register() -> u8 {
    let reply: u8 = MODBUS_HEADER_SIZE + 5; // +5 for FunctionCode, 2 Bytes RegisterAddress and 2 Bytes RegisterValue

    return reply;
}

//	===============================================================================================

#[test]
fn test_pack_telegram() {
    let test_data_1: Option<ModbusTelegram> = None;
    let result_1: Result<ModbusTelegram, String> = pack_telegram(test_data_1);
    assert!(result_1.is_err());

    let test_values: Vec<u8> = vec![0; 10];
    let test_data_2: Option<ModbusTelegram> =
        ModbusTelegram::new(0x0001, 0x01, 0x01, &test_values, 10);
    let result_2: Result<ModbusTelegram, String> = pack_telegram(test_data_2);
    assert!(result_2.is_ok());
}

fn pack_telegram(telegram: Option<ModbusTelegram>) -> Result<ModbusTelegram, String> {
    let reply: Result<ModbusTelegram, String>;

    if telegram.is_some() {
        reply = Ok(telegram.unwrap());
    } else {
        reply = Err("Error while creating telegram.".to_string());
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_coils() {
    let result: Vec<u8> = prepare_payload_read_coils(0x00FF, 0x000A);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0xFF);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x0A);
}

fn prepare_payload_read_coils(starting_address: u16, quantity_of_coils: u16) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, quantity_of_coils);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_discrete_inputs() {
    let result: Vec<u8> = prepare_payload_read_discrete_inputs(0x00FF, 0x000A);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0xFF);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x0A);
}

fn prepare_payload_read_discrete_inputs(starting_address: u16, quantity_of_inputs: u16) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, quantity_of_inputs);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_input_registers() {
    let result: Vec<u8> = prepare_payload_read_input_registers(0x00FF, 0x000A);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0xFF);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x0A);
}

fn prepare_payload_read_input_registers(
    starting_address: u16,
    quantity_of_input_registers: u16,
) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, quantity_of_input_registers);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_read_holding_registers() {
    let result: Vec<u8> = prepare_payload_read_holding_registers(0x00FF, 0x0001);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0xFF);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x01);
}

fn prepare_payload_read_holding_registers(
    starting_address: u16,
    quantity_of_registers: u16,
) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, quantity_of_registers);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_multiple_coils() {
    let output_values: Vec<u8> = vec![0x0F, 0xF0];

    let result: Vec<u8> = prepare_payload_write_multiple_coils(0x0001, 0x0010, &output_values);
    assert_eq!(result.len(), 7);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0x01);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x10);
    assert_eq!(result[4], 0x02);
    assert_eq!(result[5], 0x0F);
    assert_eq!(result[6], 0xF0);
}

fn prepare_payload_write_multiple_coils(
    starting_address: u16,
    quantity_of_outputs: u16,
    output_values: &Vec<u8>,
) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, quantity_of_outputs);
    append_byte_to_bytearray(&mut reply, output_values.len() as u8);
    append_bytearray_to_bytearray(&mut reply, &output_values);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_multiple_registers() {
    let register_values: Vec<u16> = vec![0x00FF, 0xF00F, 0x010A, 0xABCD];

    let result: Vec<u8> = prepare_payload_write_multiple_registers(0x0100, &register_values);
    assert_eq!(result.len(), 13);
    assert_eq!(result[0], 0x01);
    assert_eq!(result[1], 0x00);
    assert_eq!(result[2], 0x00);
    assert_eq!(result[3], 0x04);
    assert_eq!(result[4], 0x08);
    assert_eq!(result[5], 0x00);
    assert_eq!(result[6], 0xFF);
    assert_eq!(result[7], 0xF0);
    assert_eq!(result[8], 0x0F);
    assert_eq!(result[9], 0x01);
    assert_eq!(result[10], 0x0A);
    assert_eq!(result[11], 0xAB);
    assert_eq!(result[12], 0xCD);
}

fn prepare_payload_write_multiple_registers(
    starting_address: u16,
    register_values: &Vec<u16>,
) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, starting_address);
    append_word_to_bytearray(&mut reply, register_values.len() as u16);

    let register_values_bytes: Vec<u8> = transform_words_to_bytes(&register_values);
    append_byte_to_bytearray(&mut reply, register_values_bytes.len() as u8);
    append_bytearray_to_bytearray(&mut reply, &register_values_bytes);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_single_coil() {
    let result_1: Vec<u8> = prepare_payload_write_single_coil(0x00FF, 0x0000);
    assert_eq!(result_1.len(), 4);
    assert_eq!(result_1[0], 0x00);
    assert_eq!(result_1[1], 0xFF);
    assert_eq!(result_1[2], 0x00);
    assert_eq!(result_1[3], 0x00);

    let result_2: Vec<u8> = prepare_payload_write_single_coil(0x00FF, 0xFF00);
    assert_eq!(result_2.len(), 4);

    assert_eq!(result_2[0], 0x00);
    assert_eq!(result_2[1], 0xFF);
    assert_eq!(result_2[2], 0xFF);
    assert_eq!(result_2[3], 0x00);
}

fn prepare_payload_write_single_coil(output_address: u16, output_value: u16) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, output_address);
    append_word_to_bytearray(&mut reply, output_value);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_payload_write_single_register() {
    let result: Vec<u8> = prepare_payload_write_single_register(0x00FF, 0x010A);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 0x00);
    assert_eq!(result[1], 0xFF);
    assert_eq!(result[2], 0x01);
    assert_eq!(result[3], 0x0A);
}

fn prepare_payload_write_single_register(register_address: u16, register_value: u16) -> Vec<u8> {
    let mut reply: Vec<u8> = vec![];

    append_word_to_bytearray(&mut reply, register_address);
    append_word_to_bytearray(&mut reply, register_value);

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_coils() {
    let test_data: Vec<u8> = vec![0x03, 0xCD, 0x6B, 0x05];

    let result: Vec<bool> = prepare_response_read_coils(&test_data, 0x0013);
    assert_eq!(result.len(), 19);
    assert_eq!(result[0], true);
    assert_eq!(result[1], false);
    assert_eq!(result[2], true);
    assert_eq!(result[3], true);
    assert_eq!(result[4], false);
    assert_eq!(result[5], false);
    assert_eq!(result[6], true);
    assert_eq!(result[7], true);
    assert_eq!(result[8], true);
    assert_eq!(result[9], true);
    assert_eq!(result[10], false);
    assert_eq!(result[11], true);
    assert_eq!(result[12], false);
    assert_eq!(result[13], true);
    assert_eq!(result[14], true);
    assert_eq!(result[15], false);
    assert_eq!(result[16], true);
    assert_eq!(result[17], false);
    assert_eq!(result[18], true);
}

pub fn prepare_response_read_coils(payload: &Vec<u8>, coil_count: u16) -> Vec<bool> {
    let mut reply: Vec<bool> = vec![];

    if is_payload_read_coil_length_valid(&payload) {
        if let Some(byte_count) = extract_byte_from_bytearray(&payload, 0) {
            if let Some(coil_bytes) = extract_bytes_from_bytearray(&payload, 1, byte_count) {
                let mut byte_index: usize = 0;
                let mut bit: u8 = 0;

                for _ in 0..coil_count {
                    reply.push(coil_bytes[byte_index] & (1 << bit) != 0);

                    if bit == 7 {
                        byte_index += 1;
                        bit = 0;
                    } else {
                        bit += 1;
                    }
                }
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_discrete_inputs() {
    let test_data: Vec<u8> = vec![0x03, 0xAC, 0xDB, 0x35];

    let result: Vec<bool> = prepare_response_read_discrete_inputs(&test_data, 0x0016);
    assert_eq!(result.len(), 22);
    assert_eq!(result[0], false);
    assert_eq!(result[1], false);
    assert_eq!(result[2], true);
    assert_eq!(result[3], true);
    assert_eq!(result[4], false);
    assert_eq!(result[5], true);
    assert_eq!(result[6], false);
    assert_eq!(result[7], true);
    assert_eq!(result[8], true);
    assert_eq!(result[9], true);
    assert_eq!(result[10], false);
    assert_eq!(result[11], true);
    assert_eq!(result[12], true);
    assert_eq!(result[13], false);
    assert_eq!(result[14], true);
    assert_eq!(result[15], true);
    assert_eq!(result[16], true);
    assert_eq!(result[17], false);
    assert_eq!(result[18], true);
    assert_eq!(result[19], false);
    assert_eq!(result[20], true);
    assert_eq!(result[21], true);
}

pub fn prepare_response_read_discrete_inputs(payload: &Vec<u8>, input_count: u16) -> Vec<bool> {
    let mut reply: Vec<bool> = vec![];

    if is_payload_read_coil_length_valid(&payload) {
        if let Some(byte_count) = extract_byte_from_bytearray(&payload, 0) {
            if let Some(input_bytes) = extract_bytes_from_bytearray(&payload, 1, byte_count) {
                let mut byte_index: usize = 0;
                let mut bit: u8 = 0;

                for _ in 0..input_count {
                    reply.push(input_bytes[byte_index] & (1 << bit) != 0);

                    if bit == 7 {
                        byte_index += 1;
                        bit = 0;
                    } else {
                        bit += 1;
                    }
                }
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_holding_registers() {
    let test_data: Vec<u8> = vec![0x06, 0xF0, 0x0F, 0x00, 0xFF, 0xFF, 0x00];

    let result: Vec<u16> = prepare_response_read_holding_registers(&test_data);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], 0xF00F);
    assert_eq!(result[1], 0x00FF);
    assert_eq!(result[2], 0xFF00);
}

pub fn prepare_response_read_holding_registers(payload: &Vec<u8>) -> Vec<u16> {
    let mut reply: Vec<u16> = vec![];

    if is_payload_read_register_length_valid(&payload) {
        if let Some(byte_count) = extract_byte_from_bytearray(&payload, 0) {
            if let Some(register_values) = extract_bytes_from_bytearray(&payload, 1, byte_count) {
                let word_count: u8 = (byte_count / 2) as u8;
                reply = transform_bytes_to_words(&register_values, 0, word_count);
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_read_input_registers() {
    let test_data: Vec<u8> = vec![0x06, 0xF0, 0x0F, 0x00, 0xFF, 0xFF, 0x00];

    let result: Vec<u16> = prepare_response_read_input_registers(&test_data);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], 0xF00F);
    assert_eq!(result[1], 0x00FF);
    assert_eq!(result[2], 0xFF00);
}

pub fn prepare_response_read_input_registers(payload: &Vec<u8>) -> Vec<u16> {
    let mut reply: Vec<u16> = vec![];

    if is_payload_read_register_length_valid(&payload) {
        if let Some(byte_count) = extract_byte_from_bytearray(&payload, 0) {
            if let Some(register_values) = extract_bytes_from_bytearray(&payload, 1, byte_count) {
                let words: u8 = (byte_count / 2) as u8;
                reply = transform_bytes_to_words(&register_values, 0, words);
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_multiple_coils() {
    let test_data: Vec<u8> = vec![0x00, 0x01, 0x00, 0x03];

    let result: Vec<u16> = prepare_response_write_multiple_coils(&test_data);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], 0x0001);
    assert_eq!(result[1], 0x0003);
}

pub fn prepare_response_write_multiple_coils(payload: &Vec<u8>) -> Vec<u16> {
    let mut reply: Vec<u16> = vec![];

    if is_payload_write_length_valid(&payload) {
        reply = transform_bytes_to_words(&payload, 0, 2);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_multiple_registers() {
    let test_data: Vec<u8> = vec![0x00, 0x01, 0x00, 0xFF];

    let result: Vec<u16> = prepare_response_write_multiple_registers(&test_data);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], 0x0001);
    assert_eq!(result[1], 0x00FF);
}

pub fn prepare_response_write_multiple_registers(payload: &Vec<u8>) -> Vec<u16> {
    let mut reply: Vec<u16> = vec![];

    if is_payload_write_length_valid(&payload) {
        let option_address: Option<u16> = extract_word_from_bytearray(payload, 0);
        let option_quantity: Option<u16> = extract_word_from_bytearray(payload, 2);

        if option_address.is_some() && option_quantity.is_some() {
            reply.push(option_address.unwrap());
            reply.push(option_quantity.unwrap());
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_single_coil() {
    let test_data_1: Vec<u8> = vec![0x00, 0x01, 0xFF, 0x00];

    let result_1: Vec<bool> = prepare_response_write_single_coil(&test_data_1);
    assert_eq!(result_1.len(), 1);
    assert_eq!(result_1[0], true);

    let test_data_2: Vec<u8> = vec![0x00, 0x01, 0x01, 0x01];

    let result_2: Vec<bool> = prepare_response_write_single_coil(&test_data_2);
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0], false);

    let test_data_3: Vec<u8> = vec![0x00, 0x01, 0x0F, 0xF0];

    let result_3: Vec<bool> = prepare_response_write_single_coil(&test_data_3);
    assert_eq!(result_3.len(), 1);
    assert_eq!(result_3[0], false);
}

pub fn prepare_response_write_single_coil(payload: &Vec<u8>) -> Vec<bool> {
    let mut reply: Vec<bool> = vec![];

    if is_payload_write_length_valid(&payload) {
        if let Some(word) = extract_word_from_bytearray(payload, 2) {
            if word == 0xFF00 {
                reply.push(true);
            } else {
                reply.push(false);
            }
        }
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_prepare_response_write_single_register() {
    let test_data: Vec<u8> = vec![0x00, 0x01, 0x00, 0x03];

    let result: Vec<u16> = prepare_response_write_single_register(&test_data);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], 0x0001);
    assert_eq!(result[1], 0x0003);
}

pub fn prepare_response_write_single_register(payload: &Vec<u8>) -> Vec<u16> {
    let mut reply: Vec<u16> = vec![];

    if is_payload_write_length_valid(&payload) {
        reply = transform_bytes_to_words(&payload, 0, 2);
    }

    return reply;
}

//	===============================================================================================
#[test]
fn test_is_payload_read_coil_length_valid() {
    let test_read_data: Vec<u8> = vec![0x03, 0xAC];
    assert!(is_payload_read_coil_length_valid(&test_read_data));

    let test_read_data: Vec<u8> = vec![0x03];
    assert_eq!(is_payload_read_coil_length_valid(&test_read_data), false);
}

fn is_payload_read_coil_length_valid(payload: &Vec<u8>) -> bool {
    return payload.len() >= MODBUS_READ_COIL_MINIMUM_PAYLOAD_LENGTH;
}

//	===============================================================================================
#[test]
fn test_is_payload_read_register_length_valid() {
    let test_read_data: Vec<u8> = vec![0x03, 0xAC, 0xDB, 0x35];
    assert!(is_payload_read_register_length_valid(&test_read_data));

    let test_read_data: Vec<u8> = vec![0x03, 0xAC];
    assert_eq!(
        is_payload_read_register_length_valid(&test_read_data),
        false
    );
}

fn is_payload_read_register_length_valid(payload: &Vec<u8>) -> bool {
    return payload.len() >= MODBUS_READ_REGISTER_MINIMUM_PAYLOAD_LENGTH;
}

//	===============================================================================================
#[test]
fn test_is_payload_write_length_valid() {
    let test_write_data: Vec<u8> = vec![0x00, 0x01, 0xFF, 0x00];
    assert!(is_payload_write_length_valid(&test_write_data));

    let test_write_data: Vec<u8> = vec![0x00, 0x01];
    assert_eq!(is_payload_write_length_valid(&test_write_data), false);
}

fn is_payload_write_length_valid(payload: &Vec<u8>) -> bool {
    return payload.len() == MODBUS_WRITE_MINIMUM_PAYLOAD_LENGTH;
}

//	===============================================================================================

#[test]
fn test_is_start_and_quantity_ok() {
    assert_eq!(is_start_and_quantity_ok(0x0000, 0x00FF), true);
    assert_eq!(is_start_and_quantity_ok(0x0000, 0xFFFF), true);
    assert_eq!(is_start_and_quantity_ok(0x0100, 0x00FF), true);
    assert_eq!(is_start_and_quantity_ok(0xFFFE, 0x0001), true);
    assert_eq!(is_start_and_quantity_ok(0x0100, 0xFFFF), false);
}

fn is_start_and_quantity_ok(start: u16, quantity: u16) -> bool {
    let reply: bool;

    if (start as u32 + quantity as u32) <= 0x0000FFFF {
        reply = true;
    } else {
        reply = false;
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_is_value_in_range() {
    assert_eq!(is_value_in_range(0x0002, 0x0000, 0x000A), true);
    assert_eq!(is_value_in_range(0x0000, 0x0000, 0x000A), true);
    assert_eq!(is_value_in_range(0x000A, 0x0000, 0x000A), true);
    assert_eq!(is_value_in_range(0x000B, 0x0000, 0x000A), false);
    assert_eq!(is_value_in_range(0x0007, 0x0008, 0x000A), false);
}

fn is_value_in_range(value: u16, min_value: u16, max_value: u16) -> bool {
    let reply: bool;

    if value >= min_value && value <= max_value {
        reply = true;
    } else {
        reply = false;
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_coils() {
    let result_1: Result<bool, String> = verify_parameter_read_coils(0x0000, 0x0001);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_read_coils(0x0000, 0x07D0);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_read_coils(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_read_coils(0x0000, 0x07D1);
    assert!(result_4.is_err());

    let result_5: Result<bool, String> = verify_parameter_read_coils(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_read_coils(
    starting_address: u16,
    quantity_of_coils: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool = is_start_and_quantity_ok(starting_address, quantity_of_coils);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_coils is over 65535.".to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_coils, 0x0001, 0x07D0) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_coils == 0x0000 {
                reply = Err(
                    "Error at parameter quantity_of_coils - value to low, must be over 1."
                        .to_string(),
                );
            }

            if quantity_of_coils > 0x07D0 {
                reply = Err( "Error at parameter quantity_of_coils - value to high, must be lower or equal 2000.".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_discrete_inputs() {
    let result_1: Result<bool, String> = verify_parameter_read_discrete_inputs(0x0000, 0x0001);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_read_discrete_inputs(0x0000, 0x07D0);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_read_discrete_inputs(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_read_discrete_inputs(0x0000, 0x07D1);
    assert!(result_4.is_err());

    let result_5: Result<bool, String> = verify_parameter_read_discrete_inputs(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_read_discrete_inputs(
    starting_address: u16,
    quantity_of_inputs: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool = is_start_and_quantity_ok(starting_address, quantity_of_inputs);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_inputs is over 65535.".to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_inputs, 0x0001, 0x07D0) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_inputs == 0x0000 {
                reply = Err(
                    "Error at parameter quantity_of_inputs - value to low, must be over 1."
                        .to_string(),
                );
            }

            if quantity_of_inputs > 0x07D0 {
                reply = Err( "Error at parameter quantity_of_inputs - value to high, must be lower or equal 2000.".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_holding_registers() {
    let result_1: Result<bool, String> = verify_parameter_read_holding_registers(0x0000, 0x0001);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_read_holding_registers(0x0000, 0x007D);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_read_holding_registers(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_read_holding_registers(0x0000, 0x007E);
    assert!(result_4.is_err());

    let result_5: Result<bool, String> = verify_parameter_read_holding_registers(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_read_holding_registers(
    starting_address: u16,
    quantity_of_registers: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool = is_start_and_quantity_ok(starting_address, quantity_of_registers);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_registers is over 65535."
                .to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_registers, 0x0001, 0x007D) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_registers == 0x0000 {
                reply = Err(
                    "Error at parameter quantity_of_registers - value to low, must be over 1."
                        .to_string(),
                );
            }

            if quantity_of_registers > 0x007D {
                reply = Err( "Error at parameter quantity_of_registers - value to high, must be lower or equal 125.".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_read_input_registers() {
    let result_1: Result<bool, String> = verify_parameter_read_input_registers(0x0000, 0x0001);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_read_input_registers(0x0000, 0x007D);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_read_input_registers(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_read_input_registers(0x0000, 0x007E);
    assert!(result_4.is_err());

    let result_5: Result<bool, String> = verify_parameter_read_input_registers(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_read_input_registers(
    starting_address: u16,
    quantity_of_input_registers: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool =
        is_start_and_quantity_ok(starting_address, quantity_of_input_registers);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_input_registers is over 65535."
                .to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_input_registers, 0x0001, 0x007D) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_input_registers == 0x0000 {
                reply = Err( "Error at parameter quantity_of_input_registers - value to low, must be over 1.".to_string () );
            }

            if quantity_of_input_registers > 0x007D {
                reply = Err( "Error at parameter quantity_of_input_registers - value to high, must be lower or equal 125.".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_multiple_coils() {
    let result_1: Result<bool, String> = verify_parameter_write_multiple_coils(0x0000, 0x0001);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_write_multiple_coils(0x0000, 0x07B0);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_write_multiple_coils(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_write_multiple_coils(0x0000, 0x07B1);
    assert!(result_4.is_err(),);

    let result_5: Result<bool, String> = verify_parameter_write_multiple_coils(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_write_multiple_coils(
    starting_address: u16,
    quantity_of_output_values: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool = is_start_and_quantity_ok(starting_address, quantity_of_output_values);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_output_values is over 65535."
                .to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_output_values, 0x0001, 0x07B0) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_output_values == 0x0000 {
                reply = Err(
                    "Error at parameter quantity_of_output_values - value to low, must be over 1."
                        .to_string(),
                );
            }

            if quantity_of_output_values > 0x07B0 {
                reply = Err( "Error at parameter quantity_of_output_values - value to high, must be lower or equal 1968.".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_multiple_registers() {
    let result_1: Result<bool, String> = verify_parameter_write_multiple_registers(0x0000, 0x000F);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_write_multiple_registers(0x0000, 0x007B);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_write_multiple_registers(0x0000, 0x0000);
    assert!(result_3.is_err());

    let result_4: Result<bool, String> = verify_parameter_write_multiple_registers(0x0000, 0x007C);
    assert!(result_4.is_err());

    let result_5: Result<bool, String> = verify_parameter_write_multiple_registers(0xFFFE, 0x000F);
    assert!(result_5.is_err());
}

fn verify_parameter_write_multiple_registers(
    starting_address: u16,
    quantity_of_registers: u16,
) -> Result<bool, String> {
    let mut reply: Result<bool, String> = Ok(false);

    let address_good: bool = is_start_and_quantity_ok(starting_address, quantity_of_registers);

    if address_good {
    } else {
        reply = Err(
            "Error - range or starting_address and quantity_of_registers is over 65535."
                .to_string(),
        );
    }

    let quantity_good: bool;

    if address_good {
        if is_value_in_range(quantity_of_registers, 0x0001, 0x007B) {
            quantity_good = true;
        } else {
            quantity_good = false;

            if quantity_of_registers == 0x0000 {
                reply = Err(
                    "Error at parameter quantity_of_registers - value to low, must be over 1"
                        .to_string(),
                );
            }

            if quantity_of_registers > 0x007B {
                reply = Err( "Error at parameter quantity_of_registers - value to high, must be lower or equal 123".to_string () );
            }
        }
    } else {
        quantity_good = false;
    }

    if address_good && quantity_good {
        reply = Ok(true);
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_single_coil() {
    let result_1: Result<bool, String> = verify_parameter_write_single_coil(0x0000);
    assert!(result_1.is_ok());

    let result_2: Result<bool, String> = verify_parameter_write_single_coil(0xFF00);
    assert!(result_2.is_ok());

    let result_3: Result<bool, String> = verify_parameter_write_single_coil(0x0F0F);
    assert!(result_3.is_err());
}

fn verify_parameter_write_single_coil(output_value: u16) -> Result<bool, String> {
    let reply: Result<bool, String>;

    if output_value == 0x0000 || output_value == 0xFF00 {
        reply = Ok(true);
    } else {
        reply = Err(
            "Error at parameter output_value - valid values are only 0 [0x0000] or 65280 [0xFF00]."
                .to_string(),
        );
    }

    return reply;
}

//	===============================================================================================

#[test]
fn test_verify_parameter_write_single_register() {
    let result_1: Result<bool, String> = verify_parameter_write_single_register();
    assert!(result_1.is_ok());
}

fn verify_parameter_write_single_register() -> Result<bool, String> {
    return Ok(true);
}
