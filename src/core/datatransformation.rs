

//	===============================================================================================


#[test]
fn test_append_byte_to_bytearray ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0xFF );


	let l_byte : u8 = 0x10;

	append_byte_to_bytearray ( &mut l_test_array, l_byte );

	assert_eq! ( l_test_array.len ( ), 4 );

	assert_eq! ( l_test_array[3], l_byte );
}

pub fn append_byte_to_bytearray ( array : &mut Vec<u8>, databyte : u8 )
{	
	array.push ( databyte );		
}


//	===============================================================================================


#[test]
fn test_append_bytearray_to_bytearray ( )
{
	let mut l_test_array_target : Vec<u8> =	vec![];

	l_test_array_target.push ( 0x01 );
	l_test_array_target.push ( 0x0A );


	let mut l_test_array_source : Vec<u8> = vec![];

	l_test_array_source.push ( 0xFF );
	l_test_array_source.push ( 0x10 );


	append_bytearray_to_bytearray ( &mut l_test_array_target, &l_test_array_source );
	
	assert_eq! ( l_test_array_target.len ( ), 4 );

	assert_eq! ( l_test_array_target[2], 0xFF );
	assert_eq! ( l_test_array_target[3], 0x10 );
}

pub fn append_bytearray_to_bytearray ( target_array : &mut Vec<u8>, source_array : &Vec<u8> )
{	
	for l_byte in source_array
	{
		append_byte_to_bytearray ( target_array, *l_byte );		
	}		
}


//	===============================================================================================


#[test]
fn test_append_word_to_bytearray ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	

	let l_word : u16 = 0xFF10;

	append_word_to_bytearray ( &mut l_test_array, l_word );

	assert_eq! ( l_test_array.len ( ), 4 );

	assert_eq! ( l_test_array[2], 0xFF );
	assert_eq! ( l_test_array[3], 0x10 );
}

pub fn append_word_to_bytearray ( array : &mut Vec<u8>, dataword : u16 )
{	
	let l_splitted_word : Vec<u8> = split_word_to_bytes ( dataword );
	
	array.push ( l_splitted_word[0] );
	array.push ( l_splitted_word[1] );	
}


//	===============================================================================================


#[test]
fn test_extract_byte_from_bytearray ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0xFF );
	l_test_array.push ( 0x10 );


	let l_result_1 : Option<u8> = extract_byte_from_bytearray ( &l_test_array, 0 );

	assert! ( l_result_1.is_some ( ) );

	assert_eq! ( l_result_1.unwrap ( ), 0x01 );


	let l_result_2 : Option<u8> = extract_byte_from_bytearray ( &l_test_array, 2 );

	assert! ( l_result_2.is_some ( ) );

	assert_eq! ( l_result_2.unwrap ( ), 0xFF );


	let l_result_3 : Option<u8> = extract_byte_from_bytearray ( &l_test_array, 3 );

	assert! ( l_result_3.is_some ( ) );

	assert_eq! ( l_result_3.unwrap ( ), 0x10 );


	let l_result_4 : Option<u8> = extract_byte_from_bytearray ( &l_test_array, 4 );

	assert! ( l_result_4.is_none ( ) );
}

pub fn extract_byte_from_bytearray ( source_array : &Vec<u8>, start_index : u8 ) -> Option<u8>
{
	let l_return : Option<u8>;

	let l_result : Option<Vec<u8>> = extract_bytes_from_bytearray ( &source_array, start_index, 1 );

	if l_result.is_some ( )
	{
		l_return = Some ( l_result.unwrap ( )[0] );
	}
	else
	{
		l_return = None;
	}

	return l_return;
}


//	===============================================================================================


#[test]
fn test_extract_bytes_from_bytearray ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0xFF );
	l_test_array.push ( 0x10 );
	

	let l_result_1 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 0, 1 );

	assert! ( l_result_1.is_some ( ) );

	assert_eq! ( l_result_1.unwrap ( )[0], 0x01 );


	let l_result_2 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 1, 2 );

	assert! ( l_result_2.is_some ( ) );

	let l_result_2_data = l_result_2.unwrap ( );

	assert_eq! ( l_result_2_data[0], 0x0A );
	assert_eq! ( l_result_2_data[1], 0xFF );


	let l_result_3 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 1, 4 );

	assert! ( l_result_3.is_none ( ) );


	let l_result_4 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 3, 1 );

	assert! ( l_result_4.is_some ( ) );

	assert_eq! ( l_result_4.unwrap ( )[0], 0x10 );


	let l_result_5 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 3, 2 );

	assert! ( l_result_5.is_none ( ) );


	let l_result_6 : Option<Vec<u8>> = extract_bytes_from_bytearray ( &l_test_array, 4, 1 );

	assert! ( l_result_6.is_none ( ) );
}

pub fn extract_bytes_from_bytearray ( source_array : &Vec<u8>, start_index : u8, byte_count : u8 ) -> Option<Vec<u8>>
{
	let l_return : Option<Vec<u8>>;
	
	let l_source_length : u8 = source_array.len ( ) as u8;
	let l_verify_length : u8 = start_index + byte_count;

	if l_verify_length <= l_source_length
	{
		let mut l_copy_array : Vec<u8> = vec![];

		for l_index in start_index..l_verify_length
		{
			l_copy_array.push ( source_array [ l_index as usize ] );
		}

		if l_copy_array.len ( ) > 0
		{
			l_return = Some ( l_copy_array.clone ( ) );
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


//	===============================================================================================


#[test]
fn test_extract_word_from_bytearray ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0xFF );
	l_test_array.push ( 0x10 );


	let l_result_1 : Option<u16> = extract_word_from_bytearray ( &l_test_array, 1 );

	assert! ( l_result_1.is_some ( ) );

	assert_eq! ( l_result_1.unwrap ( ), 0x0AFF );

	
	let l_result_2 : Option<u16> = extract_word_from_bytearray ( &l_test_array, 3 );

	assert! ( l_result_2.is_none ( ) );
}

pub fn extract_word_from_bytearray ( source_array : &Vec<u8>, start_index : u8 ) -> Option<u16>
{
	let l_return : Option<u16>;

	let l_result : Option<Vec<u8>> = extract_bytes_from_bytearray ( &source_array, start_index, 2 );

	if l_result.is_some ( )
	{
		l_return = Some ( transform_bytes_to_word ( &l_result.unwrap ( ), 0 ) );
	}
	else
	{
		l_return = None;
	}

	return l_return;
}


//	===============================================================================================


#[test]
fn test_split_word_to_bytes ( )
{	
	let l_test_value_1 : u16 = 0x00F0; //	240	
	let l_test_value_2 : u16 = 0xF000; //	61440


	let l_result_value_1 : Vec<u8> = split_word_to_bytes ( l_test_value_1 );

	assert_eq! ( l_result_value_1.len ( ), 2 );

	assert_eq! ( l_result_value_1[0], 0x00 );
	assert_eq! ( l_result_value_1[1], 0xF0 );


	let l_result_value_2 : Vec<u8> = split_word_to_bytes ( l_test_value_2 );

	assert_eq! ( l_result_value_2.len ( ), 2 );

	assert_eq! ( l_result_value_2[0], 0xF0 );
	assert_eq! ( l_result_value_2[1], 0x00 );
}

/// splits a u16 value to 2 u8 values by big endian
fn split_word_to_bytes ( dataword : u16 ) -> Vec<u8>
{
	let l_lowbyte : u8 = (dataword & 0xff) as u8;

	let l_highbyte : u8 = ((dataword >> 8) & 0xff) as u8;

	return vec![ l_highbyte, l_lowbyte ];
}


//	===============================================================================================


#[test]
fn test_transform_bytes_to_word( )
{	
	let l_value : u16 = 0xF00F; //	61455

	let l_test_vec : Vec<u8> = split_word_to_bytes ( l_value );

	let l_result : u16 = transform_bytes_to_word ( &l_test_vec, 0 );

	assert_eq! ( l_result, l_value );
}

pub fn transform_bytes_to_word ( bytes : &Vec<u8>, index : u8 ) -> u16
{
	let mut l_return : u16;

	l_return = ( bytes[ index as usize ] as u16 ) << 8;
    
    l_return = l_return | ( bytes[ index as usize + 1 ] as u16 ); 

	return l_return;
}


//	===============================================================================================


#[test]
fn test_transform_bytes_to_words ( )
{
	let mut l_test_array : Vec<u8> = vec![];

	l_test_array.push ( 0x01 );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0xF0 );
	l_test_array.push ( 0x0F );
	l_test_array.push ( 0x0A );
	l_test_array.push ( 0x10 );
	l_test_array.push ( 0xFF );
	l_test_array.push ( 0xFF );


	let l_result_1 : Vec<u16> =	transform_bytes_to_words ( &l_test_array, 0, 2 );

	assert_eq! ( l_result_1.len ( ), 2 );

	assert_eq! ( l_result_1[0], 0x010A );
	assert_eq! ( l_result_1[1], 0xF00F );


	let l_result_2 : Vec<u16> =	transform_bytes_to_words ( &l_test_array, 2, 2 );

	assert_eq! ( l_result_2.len ( ), 2 );

	assert_eq! ( l_result_2[0], 0xF00F );
	assert_eq! ( l_result_2[1], 0x0A10 );


	let l_result_3 : Vec<u16> =	transform_bytes_to_words ( &l_test_array, 6, 1 );

	assert_eq! ( l_result_3.len ( ), 1 );

	assert_eq! ( l_result_3[0], 0xFFFF );


	let l_result_4 : Vec<u16> =	transform_bytes_to_words ( &l_test_array, 6, 2 );

	assert_eq! ( l_result_4.len ( ), 0 );
}

pub fn transform_bytes_to_words ( databytes : &Vec<u8>, start_index : u8, word_count :u8 ) -> Vec<u16>
{
	let mut l_return : Vec<u16> = vec![];

	let l_source_length : u8 = databytes.len ( ) as u8;
	let l_verify_length : u8 = start_index + ( word_count * 2 );

	if l_verify_length <= l_source_length
	{
		let mut l_working_index : u8 = start_index;

        let mut l_word : u16;
			
		for _ in 0..word_count
		{
			l_word = transform_bytes_to_word ( databytes, l_working_index );

			l_working_index += 2;

			l_return.push ( l_word );
		}
	}

	return l_return;
}


//	===============================================================================================


#[test]
fn test_transform_word_to_bytes ( )
{
	let l_value : u16 = 0xFFFF;

	let l_result : Vec<u8> = transform_word_to_bytes ( l_value );

	assert_eq! ( l_result.len ( ), 2 );	
}

pub fn transform_word_to_bytes ( dataword : u16 ) -> Vec<u8>
{	
	return split_word_to_bytes ( dataword );	
}


//	===============================================================================================


#[test]
fn test_transform_words_to_bytes ( )
{
	let mut l_test_array : Vec<u16> = vec![];

	l_test_array.push ( 0x010A );
	l_test_array.push ( 0xF00F );
	l_test_array.push ( 0x0A10 );
	l_test_array.push ( 0xFFFF );
	

	let l_result : Vec<u8> = transform_words_to_bytes ( &l_test_array );

	assert_eq! ( l_result.len ( ), 8 );

	assert_eq! ( l_result[0], 0x01 );
	assert_eq! ( l_result[1], 0x0A );
	assert_eq! ( l_result[2], 0xF0 );
	assert_eq! ( l_result[3], 0x0F );
	assert_eq! ( l_result[4], 0x0A );
	assert_eq! ( l_result[5], 0x10 );
	assert_eq! ( l_result[6], 0xFF );
	assert_eq! ( l_result[7], 0xFF );	
}

pub fn transform_words_to_bytes ( datawords : &Vec<u16> ) -> Vec<u8>
{
	let mut l_return : Vec<u8> = vec![];
		
    let mut l_result : Vec<u8>;

	for l_dataword in datawords
	{
		l_result = split_word_to_bytes ( *l_dataword );

		l_return.push ( l_result[0] );
		l_return.push ( l_result[1] );
	}

	return l_return;
}
