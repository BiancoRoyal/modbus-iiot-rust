

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

//	===============================================================================================

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

//	===============================================================================================

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

//	===============================================================================================

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

    pub fn get_duration_in_milliseconds ( &mut self ) -> u64
    {
        let reply : u64;

        if self.duration_in_milliseconds.is_some ()
        {
            reply = self.duration_in_milliseconds.take ().unwrap ();
        }
        else
        {
            reply = 0;
        }

        return reply;
    }    
}
