

use std::fmt; 
 
//  ===============================================================================================

#[derive( Debug )] 
pub enum ModbusReturnCoils
{
    Bad( ReturnBad ),
    Good( ReturnGood< bool > ),
    None
}

impl ModbusReturnCoils
{
    pub fn is_bad ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnCoils::Bad ( _ ) => { reply = true; }
            _                            => { reply = false; }
        }

        return reply;
    }

    pub fn is_good ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnCoils::Good( _ )    => { reply = true; }
            _                               => { reply = false; }
        }

        return reply;
    }

    pub fn is_none ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnCoils::None  => { reply = true; }
            _                        => { reply = false; }
        }

        return reply;
    }

    pub fn is_some ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnCoils::Bad( _ )     => { reply = true; }
            ModbusReturnCoils::Good( _ )    => { reply = true; }
            _                               => { reply = false; }
        }

        return reply;
    }

    pub fn unwrap_bad ( self ) -> ReturnBad
    {
        let reply : ReturnBad;
        
        match self
        {
            ModbusReturnCoils::Bad( bad )   => reply = bad,
            ModbusReturnCoils::Good( _ )    => panic! ( "called `unwrap_bad()` on a `Good` value" ),
            ModbusReturnCoils::None         => panic! ( "called `unwrap_bad()` on a `None` value" )
        };

        return reply;
    }

    pub fn unwrap_good ( self ) -> ReturnGood< bool >
    {
        let reply : ReturnGood< bool >;
        
        match self
        {
            ModbusReturnCoils::Bad( _ )     => panic! ( "called `unwrap_good()` on a `Bad` value" ),
            ModbusReturnCoils::Good( good ) => reply = good,
            ModbusReturnCoils::None         => panic! ( "called `unwrap_good()` on a `None` value" )
        };

        return reply;
    }    
}

#[test]
fn test_modbus_return_coils_on_bad ()
{
    let result : ModbusReturnCoils = ModbusReturnCoils::Bad( ReturnBad::new_with_codes ( 0x01, 
                                                                                         0x01 ) );
    assert_eq! ( result.is_none (), false );
    assert_eq! ( result.is_some (), true );
    assert_eq! ( result.is_bad (), true );
    assert_eq! ( result.is_good (), false );

    let result_data : ReturnBad = result.unwrap_bad ();
    assert_eq! ( result_data.get_error_code (), 0x01 ); 
}

#[test]
fn test_modbus_return_coils_on_good ()
{
    let test_data : Vec< bool > = vec![ true, true, false, true, false, false, true, false ];

    let result : ModbusReturnCoils = ModbusReturnCoils::Good( ReturnGood::new ( test_data,
                                                                                250 ) );
    assert_eq! ( result.is_none (), false );
    assert_eq! ( result.is_some (), true );
    assert_eq! ( result.is_bad (), false );
    assert_eq! ( result.is_good (), true );

    let mut result_good : ReturnGood< bool > = result.unwrap_good ();
    assert_eq! ( result_good.get_duration_in_milliseconds (), 250 );
    
    let result_data : Vec< bool > = result_good.get_data ();
    assert_eq! ( result_data.len (), 8 );
}

#[test]
fn test_modbus_return_coils_on_none ()
{
    let result : ModbusReturnCoils = ModbusReturnCoils::None;
    assert_eq! ( result.is_none (), true );
    assert_eq! ( result.is_some (), false );
    assert_eq! ( result.is_bad (), false );
    assert_eq! ( result.is_good (), false );
}

//	===============================================================================================

impl fmt::Display for ModbusReturnCoils 
{ 
    fn fmt ( &self, f: &mut fmt::Formatter ) -> fmt::Result 
    { 
        let state : &str; 
         
        if self.is_none () 
        { 
            state = "none"; 
        }  
        else 
        { 
            if self.is_good () 
            { 
                state = "good"; 
            } 
            else if self.is_bad () 
            { 
                state = "bad"; 
            } 
            else 
            { 
                state = "error"; 
            } 
        } 
 
        return write! ( f, "{}", state ); 
    } 
} 

//	===============================================================================================

#[derive( Debug )] 
pub enum ModbusReturnRegisters
{
    Bad( ReturnBad ),
    Good( ReturnGood< u16 > ),
    None
}

impl ModbusReturnRegisters
{
    pub fn is_bad ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnRegisters::Bad( _ ) => { reply = true; }
            _                               => { reply = false; }
        }

        return reply;
    }

    pub fn is_good ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnRegisters::Good( _ )    => { reply = true; }
            _                                   => { reply = false; }
        }

        return reply;
    }

    pub fn is_none ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnRegisters::None  => { reply = true; }
            _                            => { reply = false; }
        }

        return reply;
    }

    pub fn is_some ( &self ) -> bool
    {
        let reply : bool;

        match *self 
        {
            ModbusReturnRegisters::Bad( _ )     => { reply = true; }
            ModbusReturnRegisters::Good( _ )    => { reply = true; }
            _                                   => { reply = false; }
        }

        return reply;
    }

    pub fn unwrap_bad ( self ) -> ReturnBad
    {
        let reply : ReturnBad;
        
        match self
        {
            ModbusReturnRegisters::Bad( bad )   => reply = bad,
            ModbusReturnRegisters::Good( _ )    => panic! ( "called `unwrap_bad()` on a `Good` value" ),
            ModbusReturnRegisters::None         => panic! ( "called `unwrap_bad()` on a `None` value" )
        };

        return reply;
    }

    pub fn unwrap_good ( self ) -> ReturnGood< u16 >
    {
        let reply : ReturnGood< u16 >;
        
        match self
        {
            ModbusReturnRegisters::Bad( _ )     => panic! ( "called `unwrap_good()` on a `Bad` value" ),
            ModbusReturnRegisters::Good( good ) => reply = good,
            ModbusReturnRegisters::None         => panic! ( "called `unwrap_good()` on a `None` value" )
        };

        return reply;
    }    
}

#[test]
fn test_modbus_return_registers_on_bad ()
{
    let result : ModbusReturnRegisters = ModbusReturnRegisters::Bad( ReturnBad::new_with_codes ( 0x01, 
                                                                                                 0x01 ) );
    assert_eq! ( result.is_none (), false );
    assert_eq! ( result.is_some (), true );
    assert_eq! ( result.is_bad (), true );
    assert_eq! ( result.is_good (), false );

    let result_data : ReturnBad = result.unwrap_bad ();
    assert_eq! ( result_data.get_error_code (), 0x01 ); 
}

#[test]
fn test_modbus_return_registers_on_good ()
{
    let test_data : Vec< u16 > = vec![ 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000 ];

    let result : ModbusReturnRegisters = ModbusReturnRegisters::Good( ReturnGood::new ( test_data,
                                                                                        250 ) );
    assert_eq! ( result.is_none (), false );
    assert_eq! ( result.is_some (), true );
    assert_eq! ( result.is_bad (), false );
    assert_eq! ( result.is_good (), true );

    let mut result_good : ReturnGood< u16 > = result.unwrap_good ();
    assert_eq! ( result_good.get_duration_in_milliseconds (), 250 );
    
    let result_data : Vec< u16 > = result_good.get_data ();
    assert_eq! ( result_data.len (), 8 );
}

#[test]
fn test_modbus_return_registers_on_none ()
{
    let result : ModbusReturnRegisters = ModbusReturnRegisters::None;
    assert_eq! ( result.is_none (), true );
    assert_eq! ( result.is_some (), false );
    assert_eq! ( result.is_bad (), false );
    assert_eq! ( result.is_good (), false );
}

//	===============================================================================================

impl fmt::Display for ModbusReturnRegisters 
{ 
    fn fmt ( &self, f: &mut fmt::Formatter ) -> fmt::Result 
    { 
        let state : &str; 
         
        if self.is_none () 
        { 
            state = "none"; 
        }  
        else 
        { 
            if self.is_good () 
            { 
                state = "good"; 
            } 
            else if self.is_bad () 
            { 
                state = "bad"; 
            } 
            else 
            { 
                state = "error"; 
            } 
        } 
 
        return write! ( f, "{}", state ); 
    } 
} 

//	===============================================================================================

#[derive( Debug )] 
pub struct ReturnBad
{
    error_code : Option< u8 >,
    exception_code : Option< u8 >,
    message : Option< String >
}

impl ReturnBad
{
    pub fn new_with_codes ( error_code : u8, exception_code : u8 ) -> ReturnBad
    {
        let reply : ReturnBad = ReturnBad
                                {
                                    error_code : Some( error_code ),
                                    exception_code : Some( exception_code ),
                                    message : get_message_of_exception_code ( exception_code )
                                };

        return reply;
    }

    pub fn new_with_message ( message : &str ) -> ReturnBad
    {
        let reply : ReturnBad = ReturnBad
                                {
                                    error_code : None,
                                    exception_code : None,
                                    message : Some( message.to_owned ( ) )
                                };

        return reply;
    }

    pub fn get_error_code ( &self ) -> u8
    {
        let mut reply : u8 = 0x00;

        if self.error_code.is_some ()
        {
            reply = self.error_code.unwrap ();            
        }

        return reply;
    }

    pub fn get_exception_code ( &self ) -> u8
    {
        let mut reply : u8 = 0x00;

        if self.exception_code.is_some ()
        {
            reply = self.exception_code.unwrap ();            
        }

        return reply;
    }

    pub fn get_message ( self ) -> String
    {
        let mut reply : String = "Err".to_string ();

        if self.message.is_some ()
        {
            reply = self.message.unwrap ();
        }

        return reply;
    }
}

#[test]
fn test_return_bad_by_new_with_codes ()
{
    let result : ReturnBad = ReturnBad::new_with_codes ( 0x01, 
                                                         0x01 );
    assert_eq! ( result.get_error_code (), 0x01 );
    assert_eq! ( result.get_exception_code (), 0x01 );
    assert! ( result.get_message ().len () > 0 );
}

#[test]
fn test_return_bad_by_new_with_message ()
{
    let message : &str = "test message";
    let result : ReturnBad = ReturnBad::new_with_message ( message );
    assert_eq! ( result.get_error_code (), 0x00 );
    assert_eq! ( result.get_exception_code (), 0x00 );
    assert_eq! ( result.get_message (), message );
}

//	===============================================================================================

#[test]
fn test_get_message_of_exception_code ()
{
    let result_1 : Option< String > = get_message_of_exception_code ( 0x01 );
    assert! ( result_1.is_some () );

    let result_2 : Option< String > = get_message_of_exception_code ( 0x02 );
    assert! ( result_2.is_some () );

    let result_3 : Option< String > = get_message_of_exception_code ( 0x03 );
    assert! ( result_3.is_some () );

    let result_4 : Option< String > = get_message_of_exception_code ( 0x04 );
    assert! ( result_4.is_some () );
}

fn get_message_of_exception_code ( code : u8 ) -> Option< String >
{
    let reply : Option< String >;
 
    let message : String =
        match code
        {
            0x01 => "Exception Code 01 - ILLEGAL FUNCTION".to_string (),
            0x02 => "Exception Code 02 - ILLEGAL DATA ADDRESS".to_string (),
            0x03 => "Exception Code 03 - ILLEGAL DATA VALUE".to_string (),
            0x04 => "Exception Code 04 - SERVER DEVICE FAILURE".to_string (),
            _ => "Unknown Exception Code".to_string ()
        };

    reply = Some( message );

    return reply;
}

//	===============================================================================================

#[derive( Debug )] 
pub struct ReturnGood< T >
{
    data : Option< Vec< T > >,
    duration_in_milliseconds : Option< u64 >
}

impl< T > ReturnGood< T >
{
    pub fn new ( data : Vec< T >, duration_in_milliseconds : u64 ) -> ReturnGood< T >
    {
        let reply : ReturnGood< T > = ReturnGood
                                      {
                                          data : Some( data ),
                                          duration_in_milliseconds : Some( duration_in_milliseconds )
                                      };
        
        return reply;
    }

    pub fn get_data ( &mut self ) -> Vec< T >
    {
        let reply : Vec< T >;

        if self.data.is_some ()
        {
            reply = self.data.take ().unwrap ();
        }
        else
        {
            reply = vec![];
        }

        return reply;
    }

    pub fn get_duration_in_milliseconds ( &self ) -> u64
    {
        let reply : u64;

        if self.duration_in_milliseconds.is_some ()
        {
            reply = *self.duration_in_milliseconds.as_ref ().unwrap ();
        }
        else
        {
            reply = 0;
        }

        return reply;
    }    
}

#[test]
fn test_return_good ()
{
    let test_data : Vec< u16 > = vec![ 0xFF00, 0x00FF ];

    let mut result : ReturnGood< u16 > = ReturnGood::new ( test_data,
                                                           4 );
    assert_eq! ( result.get_duration_in_milliseconds (), 4 );
    
    let result_data : Vec< u16 > = result.get_data ();
    assert_eq! ( result_data.len (), 2 );
    assert_eq! ( result_data[ 0 ], 0xFF00 );
    assert_eq! ( result_data[ 1 ], 0x00FF );
}
