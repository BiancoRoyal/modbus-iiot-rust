

use std::io::{Write, Read};
use std::net::TcpStream;
use core::modbustelegram::ModbusTelegram;

//	===============================================================================================

fn read_telegram_from_stream ( stream : &mut TcpStream, expected_bytes : u8 ) -> Result< ModbusTelegram, String >
{
	let mut reply : Result< ModbusTelegram, String > = Err( "Tcp Read Failed".to_string () );

	let mut data : Vec< u8 > = vec![ 0; expected_bytes as usize ];
	let response = stream.read ( &mut data );

	if response.is_ok ()
	{
        if let Some( telegram ) = ModbusTelegram::new_from_bytes ( &data )
        {
            reply = Ok( telegram );
        }
	}

	return reply;
}

//	===============================================================================================

fn write_telegram_to_stream ( stream : &mut TcpStream, telegram : &ModbusTelegram ) -> Result< bool, String >
{
	let mut reply : Result< bool, String > = Err( "Tcp Write Failed".to_string () );

	if let Some( bytes ) = telegram.get_bytes ()
	{
		let response = stream.write_all ( &bytes );
		
		if response.is_ok ()
		{
			reply = Ok( true );
		}
	}

	return reply;
}

//	===============================================================================================

pub fn process_modbus_telegram ( stream : &mut TcpStream, telegram : &Option< ModbusTelegram > ) -> Option< ModbusTelegram >
{
	let mut reply : Option< ModbusTelegram > = None;

	if telegram.is_some ()
	{
		let write_telegram : &ModbusTelegram = telegram.as_ref ().unwrap ();
		let write_response : Result< bool, String > = write_telegram_to_stream ( stream, 
																				  write_telegram );
		if write_response.is_ok ()
		{
			let expected_bytes : Option< u8 > = write_telegram.get_expected_byte_count ();
			let read_response : Result< ModbusTelegram, String > = read_telegram_from_stream ( stream, 
																							   expected_bytes.unwrap () );
			if read_response.is_ok ()
			{
				reply = Some( read_response.unwrap ( ) );
			}
		}
	}
	
	return reply;
}
