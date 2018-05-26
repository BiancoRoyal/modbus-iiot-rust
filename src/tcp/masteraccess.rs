

//  ===============================================================================================

const WRITE_SINGLE_COIL_OFF : u16 = 0x0000;
const WRITE_SINGLE_COIL_ON : u16 = 0xFF00;

//  ===============================================================================================

#[derive ( Clone, Copy, Debug, PartialEq )]
pub enum CoilValue
{
    Off,
    On
}

impl CoilValue
{
    pub fn get ( coil : &CoilValue ) -> bool
    {
        return *coil == CoilValue::On;
    }

    pub fn set ( on : bool ) -> CoilValue
    {
        let reply : CoilValue;

        if on
        {
            reply = CoilValue::On;
        }
        else
        {
            reply = CoilValue::Off;
        }

        return reply;
    }    
}

//  ===============================================================================================

#[test]
fn test_convert_for_write_single_coil ()
{
    let test_data_1 : CoilValue = CoilValue::On;
    let result_1 : u16 = convert_for_write_single_coil ( &test_data_1 );
    assert_eq! ( result_1, WRITE_SINGLE_COIL_ON );

    let test_data_2 : CoilValue = CoilValue::Off;
    let result_2 : u16 = convert_for_write_single_coil ( &test_data_2 );
    assert_eq! ( result_2, WRITE_SINGLE_COIL_OFF );
}

pub fn convert_for_write_single_coil ( coil : &CoilValue ) -> u16
{
    let reply : u16;

    if CoilValue::get ( coil )
    {
        reply = WRITE_SINGLE_COIL_ON;
    }
    else
    {
        reply = WRITE_SINGLE_COIL_OFF;
    }

    return reply;
}

//  ===============================================================================================

/// MODBUS API (for direct use)
pub trait MasterAccess
{
    ///	MODBUS Function Code 1 (0x01)
    /// This function code is used to read from 1 to 2000 contiguous 
    /// status of coils in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::{MasterAccess, CoilValue};
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let length : u16 = 16;
    /// 
    ///     let response = client.read_coils(start_address, length);
    ///     println!("Response of read_coils: {:?}", response);
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn read_coils ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >;

    ///	MODBUS Function Code 2 (0x02)
    /// This function code is used to read from 1 to 2000 contiguous 
    /// status of discrete inputs in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::{MasterAccess, CoilValue};
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let length : u16 = 16;
    /// 
    ///     let response = client.read_discrete_inputs(start_address, length);
    ///     println!("Response of read_discrete_inputs: {:?}", response);
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn read_discrete_inputs ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >;

    ///	MODBUS Function Code 3 (0x03)
    /// This function code is used to read the contents of a contiguous
    /// block of holding registers in a remote  device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::MasterAccess;
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let length : u16 = 16;
    /// 
    ///     let response = client.read_holding_registers(start_address, length);
    ///     println!("Response of read_holding_registers: {:?}", response);
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn read_holding_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >;

    ///	MODBUS Function Code 4 (0x04)
    /// This function code is used to read from 1 to 125 contiguous 
    /// input registers in a remote device. 
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::MasterAccess;
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let length : u16 = 16;
    /// 
    ///     let response = client.read_input_registers(start_address, length);
    ///     println!("Response of read_input_registers: {:?}", response);
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn read_input_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >;

    /// MODBUS Function Code 5 (0x05)
    /// This function code is used to write a single output to 
    /// either ON or OFF in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::{MasterAccess, CoilValue};
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     client.write_single_coil(64, CoilValue::set(true));
    ///     client.write_single_coil(65, CoilValue::set(false));
    ///     client.write_single_coil(66, CoilValue::On);
    ///     client.write_single_coil(67, CoilValue::Off);
    /// 
    ///     client.disconnect();        
    /// }
    /// ```
    fn write_single_coil ( &mut self, address : u16, value : CoilValue ) -> bool;

    ///	MODBUS Function Code 6 (0x06)
    /// This function code is used to write a single 
    /// holding register in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::MasterAccess;
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    /// 
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let value : u16 = 12345;
    /// 
    ///     client.write_single_register(start_address, value);
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn write_single_register ( &mut self, address : u16, value : u16 ) -> bool;

    ///	MODBUS Function Code 15 (0x0F)
    /// This function code is used to force each coil in a 
    /// sequence of coils to either ON or OFF in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::{MasterAccess, CoilValue};
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let values : Vec<CoilValue> = vec![ CoilValue::On, CoilValue::Off, CoilValue::On, CoilValue::On,
    ///                                         CoilValue::Off, CoilValue::Off, CoilValue::On, CoilValue::On,
    ///                                         CoilValue::On, CoilValue::Off ];
    /// 
    ///     client.write_multiple_coils(start_address, values );
    /// 
    ///     client.disconnect();    
    /// }
    /// ```
    fn write_multiple_coils ( &mut self, address : u16, values : Vec< CoilValue > ) -> bool;

    ///	MODBUS Function Code 16 (0x10)
    /// This function code is used to write a block of contiguous
    /// registers (1 to 123 registers) in a remote device. 
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// 
    /// use modbus_iiot::tcp::master::TcpClient;
    /// use modbus_iiot::tcp::masteraccess::MasterAccess;
    /// 
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let start_address : u16 = 64;
    ///     let values : Vec<u16> = vec![ 12345, 1, 6789, 42 ];
    /// 
    ///     client.write_multiple_registers(start_address, values);
    ///    
    ///     client.disconnect ();    
    /// }
    /// ```
    fn write_multiple_registers ( &mut self, address : u16, values : Vec< u16 > ) -> bool;
}

//  ===============================================================================================

#[test]
fn test_transform_coils_to_bytearray ()
{
    let test_data : Vec< CoilValue > = vec![ CoilValue::On, CoilValue::Off, CoilValue::On, CoilValue::On,
                                             CoilValue::Off, CoilValue::Off, CoilValue::On, CoilValue::On,
                                             CoilValue::On, CoilValue::Off ];
    let result : Vec< u8 > = transform_coils_to_bytearray ( &test_data );
    assert_eq! ( result.len (), 2 );
    assert_eq! ( result[ 0 ], 0xCD );
    assert_eq! ( result[ 1 ], 0x01 );
}

pub fn transform_coils_to_bytearray ( coils : &Vec< CoilValue > ) -> Vec< u8 >
{
	let mut reply : Vec< u8 > = vec![];

	if coils.len () > 0
	{
        reply.push ( 0x00 );
		let mut index : usize = 0;
		let mut bit : u8 = 0x00;

		for coil in coils
		{
			if CoilValue::get ( coil )
			{
				reply[ index ] |= 1 << bit;
			}
			
			if bit == 7
			{                
				reply.push ( 0x00 );
                
                index += 1;
				bit = 0x00;
			}
			else
			{
				bit += 1;
			}
		}
	}

	return reply;
}
