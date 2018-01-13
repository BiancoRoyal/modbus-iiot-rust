

use std::io::{Write, Read};
use std::net::TcpStream;
use core::modbustelegram::ModbusTelegram;


const MODBUS_HEADER_SIZE : u16 = 0x0007;


//	===============================================================================================


fn read_telegram_from_stream ( stream : &mut TcpStream, expected_bytes : u8 ) -> Result<ModbusTelegram, String>
{
	let mut l_return : Result<ModbusTelegram, String> =	Err ( "Tcp Read Failed".to_string ( ) );
        
	//	l_data muss mit der genauen Länge der Bytes initialisiert werden die gelesen werden sollen
	let mut l_data : Vec<u8> = vec![ 0; expected_bytes as usize ];

	let l_io_response = stream.read ( &mut l_data );

	//	Auswertung des Response in Funktion auslagern
	if l_io_response.is_ok()
	{
		//	start debug ausgabe
		println! ( "=> read_telegram_from_stream" );
		print_array_of_u8 ( &l_data );
		//	ende debug ausgabe

		let l_telegram : Option<ModbusTelegram> = ModbusTelegram::new_from_bytes ( &l_data );

        if l_telegram.is_some ( )
        {
            l_return = Ok ( l_telegram.unwrap ( ) );
        }
	}

	return l_return;
}


//	===============================================================================================


fn write_telegram_to_stream ( stream : &mut TcpStream, telegram : &ModbusTelegram ) -> Result<bool, String>
{
	let mut l_return : Result<bool, String> = Err ( "Tcp Write Failed".to_string ( ) );

	let l_data : Vec<u8>;
	
	let l_bytes : Option<Vec<u8>> = telegram.get_bytes ( );

	if l_bytes.is_some ( )
	{
		l_data = l_bytes.unwrap ( );

		//	start debug ausgabe
		println! ( "=> write_telegram_to_stream" );
		print_array_of_u8 ( &l_data );
		//	ende debug ausgabe

		let l_response = stream.write_all ( &l_data );

		if l_response.is_ok ( )
		{
			l_return = Ok ( true );
		}
	}

	return l_return;
}


//	===============================================================================================


pub fn process_modbus_telegram ( stream : &mut TcpStream, l_telegram_option : &mut Option<ModbusTelegram> ) -> Option<ModbusTelegram>
{
	let l_return : Option<ModbusTelegram>;

	if l_telegram_option.is_some ( )
	{
		let l_telegram_write : &ModbusTelegram = l_telegram_option.as_ref ( ).unwrap ( );

		let l_write_response : Result<bool, String> = write_telegram_to_stream ( stream, &l_telegram_write );

		if l_write_response.is_ok ( )
		{
			let l_expected_bytes : Option<u8> =	l_telegram_write.get_expected_byte_count ( );

			let l_read_response : Result<ModbusTelegram, String> = read_telegram_from_stream ( stream, l_expected_bytes.unwrap ( ) );

			if l_read_response.is_ok ( )
			{
				l_return = Some ( l_read_response.unwrap ( ) );
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
	}
	else
	{
		l_return = None;
	}
	
	return l_return;
}


fn print_array_of_u8 ( array : &Vec<u8> )
{
	print!("size of byte-array {} => ", array.len ( ) );

	for byte in array
	{		
		print! ( "{}, ", byte );		
	}

	println! ( );
}
