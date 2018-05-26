

//	===============================================================================================

#[test]
fn test_append_byte_to_bytearray ()
{
	let mut test_array : Vec< u8 > = vec![ 0x01, 0x0A, 0xFF];
	let byte : u8 = 0x10;

	append_byte_to_bytearray ( &mut test_array, 
							   byte );
	assert_eq! ( test_array.len (), 4 );
	assert_eq! ( test_array[ 3 ], byte );
}

pub fn append_byte_to_bytearray ( array : &mut Vec< u8 >, databyte : u8 )
{	
	array.push ( databyte );		
}

//	===============================================================================================

#[test]
fn test_append_bytearray_to_bytearray ()
{
	let mut test_array_target : Vec< u8 > = vec![ 0x01, 0x0A ];
	let test_array_source : Vec< u8 > = vec![ 0xFF, 0x10 ];

	append_bytearray_to_bytearray ( &mut test_array_target, 
									&test_array_source );
	assert_eq! ( test_array_target.len (), 4 );
	assert_eq! ( test_array_target[ 2 ], 0xFF );
	assert_eq! ( test_array_target[ 3 ], 0x10 );
}

pub fn append_bytearray_to_bytearray ( target_array : &mut Vec< u8 >, source_array : &Vec< u8 > )
{	
	for byte in source_array
	{
		append_byte_to_bytearray ( target_array, 
								   *byte );		
	}		
}

//	===============================================================================================

#[test]
fn test_append_word_to_bytearray ()
{
	let mut test_array : Vec< u8 > = vec![ 0x01, 0x0A ];
	let word : u16 = 0xFF10;

	append_word_to_bytearray ( &mut test_array, 
							   word );
	assert_eq! ( test_array.len (), 4 );
	assert_eq! ( test_array[ 2 ], 0xFF );
	assert_eq! ( test_array[ 3 ], 0x10 );
}

pub fn append_word_to_bytearray ( array : &mut Vec< u8 >, dataword : u16 )
{	
	let splitted_word : Vec< u8 > = split_word_to_bytes ( dataword );
	
	array.push ( splitted_word[ 0 ] );
	array.push ( splitted_word[ 1 ] );	
}

//	===============================================================================================

#[test]
fn test_extract_byte_from_bytearray ()
{
	let test_array : Vec< u8 > = vec![ 0x01, 0x0A, 0xFF, 0x10 ];

	let result_1 : Option< u8 > = extract_byte_from_bytearray ( &test_array, 
																0 );
	assert! ( result_1.is_some () );
	assert_eq! ( result_1.unwrap (), 0x01 );

	let result_2 : Option< u8 > = extract_byte_from_bytearray ( &test_array, 
																2 );
	assert! ( result_2.is_some () );
	assert_eq! ( result_2.unwrap (), 0xFF );

	let result_3 : Option< u8 > = extract_byte_from_bytearray ( &test_array, 
																3 );
	assert! ( result_3.is_some () );
	assert_eq! ( result_3.unwrap (), 0x10 );

	let result_4 : Option< u8 > = extract_byte_from_bytearray ( &test_array, 
																4 );
	assert! ( result_4.is_none () );
}

pub fn extract_byte_from_bytearray ( source_array : &Vec< u8 >, start_index : u8 ) -> Option< u8 >
{
	let reply : Option< u8 >;

	if let Some( result ) = extract_bytes_from_bytearray ( &source_array, 
														   start_index, 
														   1 )
	{
		reply = Some( result[ 0 ] );
	}
	else
	{
		reply = None;
	}

	return reply;
}

//	===============================================================================================

#[test]
fn test_extract_bytes_from_bytearray ()
{
	let test_array : Vec< u8 > = vec![ 0x01, 0x0A, 0xFF, 0x10 ];

	let result_1 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		0, 
																		1 );
	assert! ( result_1.is_some () );
	assert_eq! ( result_1.unwrap ()[ 0 ], 0x01 );

	let result_2 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		1, 
																		2 );
	assert! ( result_2.is_some () );

	let result_2_data = result_2.unwrap ();
	assert_eq! ( result_2_data[ 0 ], 0x0A );
	assert_eq! ( result_2_data[ 1 ], 0xFF );

	let result_3 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		1, 
																		4 );
	assert! ( result_3.is_none () );

	let result_4 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		3, 
																		1 );
	assert! ( result_4.is_some () );
	assert_eq! ( result_4.unwrap ()[ 0 ], 0x10 );

	let result_5 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		3, 
																		2 );
	assert! ( result_5.is_none () );

	let result_6 : Option< Vec< u8 > > = extract_bytes_from_bytearray ( &test_array, 
																		4, 
																		1 );
	assert! ( result_6.is_none () );
}

pub fn extract_bytes_from_bytearray ( source_array : &Vec< u8 >, start_index : u8, byte_count : u8 ) -> Option< Vec< u8 > >
{
	let reply : Option< Vec< u8 > >;
	
	let source_length : u8 = source_array.len () as u8;
	let verify_length : u8 = start_index + byte_count;

	if verify_length <= source_length
	{
		let mut copy_array : Vec< u8 > = vec![];

		for index in start_index..verify_length
		{
			copy_array.push ( source_array [ index as usize ] );
		}

		if copy_array.len () > 0
		{
			reply = Some( copy_array.clone () );
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

//	===============================================================================================

#[test]
fn test_extract_word_from_bytearray ()
{
	let test_array : Vec< u8 > = vec![ 0x01, 0x0A, 0xFF, 0x10 ];

	let result_1 : Option< u16 > = extract_word_from_bytearray ( &test_array, 
																 1 );
	assert! ( result_1.is_some () );
	assert_eq! ( result_1.unwrap (), 0x0AFF );
	
	let result_2 : Option< u16 > = extract_word_from_bytearray ( &test_array, 
																 3 );
	assert! ( result_2.is_none () );
}

pub fn extract_word_from_bytearray ( source_array : &Vec< u8 >, start_index : u8 ) -> Option< u16 >
{
	let reply : Option< u16 >;

	if let Some( result ) = extract_bytes_from_bytearray ( &source_array, 
														   start_index, 
														   2 )
	{
		reply = Some( 
			transform_bytes_to_word ( &result, 
									  0 )
		);
	}
	else
	{
		reply = None;
	}

	return reply;
}

//	===============================================================================================

#[test]
fn test_split_word_to_bytes ()
{	
	let test_value_1 : u16 = 0x00F0;
	let test_value_2 : u16 = 0xF000;

	let result_value_1 : Vec< u8 > = split_word_to_bytes ( test_value_1 );
	assert_eq! ( result_value_1.len (), 2 );
	assert_eq! ( result_value_1[ 0 ], 0x00 );
	assert_eq! ( result_value_1[ 1 ], 0xF0 );

	let result_value_2 : Vec< u8 > = split_word_to_bytes ( test_value_2 );
	assert_eq! ( result_value_2.len (), 2 );
	assert_eq! ( result_value_2[ 0 ], 0xF0 );
	assert_eq! ( result_value_2[ 1 ], 0x00 );
}

fn split_word_to_bytes ( dataword : u16 ) -> Vec< u8 >
{
	let lowbyte : u8 = ( dataword & 0xff ) as u8;
	let highbyte : u8 = ( ( dataword >> 8 ) & 0xff ) as u8;

	return vec![ highbyte, lowbyte ];
}

//	===============================================================================================

#[test]
fn test_transform_bytes_to_word ()
{	
	let value : u16 = 0xF00F;
	let test_vec : Vec< u8 > = split_word_to_bytes ( value );

	let result : u16 = transform_bytes_to_word ( &test_vec, 
												 0 );
	assert_eq! ( result, value );
}

pub fn transform_bytes_to_word ( bytes : &Vec< u8 >, index : u8 ) -> u16
{
	let mut reply : u16;

	reply = ( bytes[ index as usize ] as u16 ) << 8;
    reply = reply | ( bytes[ index as usize + 1 ] as u16 ); 

	return reply;
}

//	===============================================================================================

#[test]
fn test_transform_bytes_to_words ()
{
	let test_array : Vec< u8 > = vec![ 0x01, 0x0A, 0xF0, 0x0F, 0x0A, 0x10, 0xFF, 0xFF ];

	let result_1 : Vec< u16 > =	transform_bytes_to_words ( &test_array, 
														   0,
														   2 );
	assert_eq! ( result_1.len (), 2 );
	assert_eq! ( result_1[ 0 ], 0x010A );
	assert_eq! ( result_1[ 1 ], 0xF00F );

	let result_2 : Vec< u16 > =	transform_bytes_to_words ( &test_array, 
														   2, 
														   2 );
	assert_eq! ( result_2.len (), 2 );
	assert_eq! ( result_2[ 0 ], 0xF00F );
	assert_eq! ( result_2[ 1 ], 0x0A10 );

	let result_3 : Vec< u16 > =	transform_bytes_to_words ( &test_array, 
														   6, 
														   1 );
	assert_eq! ( result_3.len (), 1 );
	assert_eq! ( result_3[ 0 ], 0xFFFF );

	let result_4 : Vec< u16 > =	transform_bytes_to_words ( &test_array, 
														   6, 
														   2 );
	assert_eq! ( result_4.len (), 0 );
}

pub fn transform_bytes_to_words ( databytes : &Vec< u8 >, start_index : u8, word_count :u8 ) -> Vec< u16 >
{
	let mut reply : Vec< u16 > = vec![];

	let source_length : u8 = databytes.len () as u8;
	let verify_length : u8 = start_index + ( word_count * 2 );

	if verify_length <= source_length
	{
		let mut working_index : u8 = start_index;
        let mut word : u16;
			
		for _ in 0..word_count
		{
			word = transform_bytes_to_word ( databytes, 
											 working_index );
			working_index += 2;

			reply.push ( word );
		}
	}

	return reply;
}

//	===============================================================================================

#[test]
fn test_transform_word_to_bytes ()
{
	let value : u16 = 0xFFFF;

	let result : Vec< u8 > = transform_word_to_bytes ( value );
	assert_eq! ( result.len (), 2 );	
}

pub fn transform_word_to_bytes ( dataword : u16 ) -> Vec< u8 >
{	
	return split_word_to_bytes ( dataword );	
}

//	===============================================================================================

#[test]
fn test_transform_words_to_bytes ()
{
	let test_array : Vec< u16 > = vec![ 0x010A, 0xF00F, 0x0A10, 0xFFFF ];

	let result : Vec< u8 > = transform_words_to_bytes ( &test_array );
	assert_eq! ( result.len (), 8 );
	assert_eq! ( result[ 0 ], 0x01 );
	assert_eq! ( result[ 1 ], 0x0A );
	assert_eq! ( result[ 2 ], 0xF0 );
	assert_eq! ( result[ 3 ], 0x0F );
	assert_eq! ( result[ 4 ], 0x0A );
	assert_eq! ( result[ 5 ], 0x10 );
	assert_eq! ( result[ 6 ], 0xFF );
	assert_eq! ( result[ 7 ], 0xFF );	
}

pub fn transform_words_to_bytes ( datawords : &Vec< u16 > ) -> Vec< u8 >
{
	let mut reply : Vec< u8 > = vec![];
		
    let mut result : Vec< u8 >;

	for dataword in datawords
	{
		result = split_word_to_bytes ( *dataword );

		reply.push ( result[ 0 ] );
		reply.push ( result[ 1 ] );
	}

	return reply;
}
