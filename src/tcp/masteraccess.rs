

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
    pub fn get ( coil : CoilValue ) -> bool
    {
        return coil == CoilValue::On;
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

pub fn convert_for_write_single_coil ( coil : CoilValue ) -> u16
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
    /// Function Code 5 (0x05)
    fn write_single_coil ( &mut self, address : u16, value : CoilValue ) -> bool;
}
