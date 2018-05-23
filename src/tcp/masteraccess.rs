

//  ===============================================================================================

const WRITE_SINGLE_COIL_OFF : u16 = 0x0000;
const WRITE_SINGLE_COIL_ON : u16 = 0xFF00;

//  ===============================================================================================

#[derive ( Debug, Clone, Copy, PartialEq )]
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

pub trait MasterAccess
{
    ///	Function Code 1 (0x01)
    fn read_coils ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >;

    ///	Function Code 2 (0x02)
    fn read_discrete_inputs ( &mut self, address : u16, quantity : u16 ) -> Vec< CoilValue >;

    ///	Function Code 3 (0x03)
    fn read_holding_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >;

    ///	Function Code 4 (0x04)
    fn read_input_registers ( &mut self, address : u16, quantity : u16 ) -> Vec< u16 >;

    /// Function Code 5 (0x05)
    fn write_single_coil ( &mut self, address : u16, value : CoilValue ) -> bool;

    ///	Function Code 6 (0x06)
    fn write_single_register ( &mut self, address : u16, value : u16 ) -> bool;

    ///	Function Code 15 (0x0F)
    fn write_multiple_coils ( &mut self, address : u16, values : Vec< CoilValue > ) -> bool;

    ///	Function Code 16 (0x10)
    fn write_multiple_registers ( &mut self, address : u16, values : Vec< u16 > ) -> bool;
}

//  ===============================================================================================

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
    print_bytes ( &reply );
	return reply;
}

fn print_bytes ( bytes : &Vec< u8 > )
{
    for byte in bytes
    {
        println!("{}", byte );
    }
}