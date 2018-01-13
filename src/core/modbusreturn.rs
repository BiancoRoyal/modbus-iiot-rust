

pub enum ModbusReturnCoils
{
    Bad ( ReturnBad ),

    Good ( ReturnGood<bool> ),

    None
}


impl ModbusReturnCoils
{

    pub fn is_bad ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnCoils::Bad ( _ ) => { l_return = true; }
            _                            => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_good ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnCoils::Good ( _ )    => { l_return = true; }
            _                                => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_none ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnCoils::None  => { l_return = true; }
            _                        => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_some ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnCoils::Bad ( _ )     => { l_return = true; }
            ModbusReturnCoils::Good ( _ )    => { l_return = true; }
            _                                => { l_return = false; }
        }

        return l_return;
    }

    pub fn unwrap_bad ( self ) -> ReturnBad
    {
        let l_return : ReturnBad;
        
        match self
        {
            ModbusReturnCoils::Bad ( l_bad ) => l_return = l_bad,
            ModbusReturnCoils::Good ( _ )    => panic! ( "called `unwrap_bad()` on a `Good` value" ),
            ModbusReturnCoils::None          => panic! ( "called `unwrap_bad()` on a `None` value" )
        };

        return l_return;
    }

    pub fn unwrap_good ( self ) -> ReturnGood<bool>
    {
        let l_return : ReturnGood<bool>;
        
        match self
        {
            ModbusReturnCoils::Bad ( _ )         => panic! ( "called `unwrap_good()` on a `Bad` value" ),
            ModbusReturnCoils::Good ( l_good )   => l_return = l_good,
            ModbusReturnCoils::None              => panic! ( "called `unwrap_good()` on a `None` value" )
        };

        return l_return;
    }
    
}


pub enum ModbusReturnRegisters
{
    Bad ( ReturnBad ),

    Good ( ReturnGood<u16> ),

    None
}


impl ModbusReturnRegisters
{

    pub fn is_bad ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnRegisters::Bad ( _ ) => { l_return = true; }
            _                                => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_good ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnRegisters::Good ( _ )    => { l_return = true; }
            _                                    => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_none ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnRegisters::None  => { l_return = true; }
            _                            => { l_return = false; }
        }

        return l_return;
    }

    pub fn is_some ( &self ) -> bool
    {
        let l_return : bool;

        match *self 
        {
            ModbusReturnRegisters::Bad ( _ )     => { l_return = true; }
            ModbusReturnRegisters::Good ( _ )    => { l_return = true; }
            _                                    => { l_return = false; }
        }

        return l_return;
    }

    pub fn unwrap_bad ( self ) -> ReturnBad
    {
        let l_return : ReturnBad;
        
        match self
        {
            ModbusReturnRegisters::Bad ( l_bad ) => l_return = l_bad,
            ModbusReturnRegisters::Good ( _ )    => panic! ( "called `unwrap_bad()` on a `Good` value" ),
            ModbusReturnRegisters::None          => panic! ( "called `unwrap_bad()` on a `None` value" )
        };

        return l_return;
    }

    pub fn unwrap_good ( self ) -> ReturnGood<u16>
    {
        let l_return : ReturnGood<u16>;
        
        match self
        {
            ModbusReturnRegisters::Bad ( _ )         => panic! ( "called `unwrap_good()` on a `Bad` value" ),
            ModbusReturnRegisters::Good ( l_good )   => l_return = l_good,
            ModbusReturnRegisters::None              => panic! ( "called `unwrap_good()` on a `None` value" )
        };

        return l_return;
    }
    
}

#[repr(C)]
pub struct ReturnBad
{

    error_code : Option<u8>,

    exception_code : Option<u8>,

    message : Option<String>

}


impl ReturnBad
{

    pub fn get_error_code ( &self ) -> u8
    {
        let mut l_return : u8 = 0x00;

        if self.error_code.is_some ( )
        {
            l_return = self.error_code.unwrap ( );            
        }

        return l_return;
    }

    pub fn get_message ( self ) -> String
    {
        let mut l_return : String = "Err".to_string ( );

        if self.message.is_some ( )
        {
            l_return = self.message.unwrap ( );
        }

        return l_return;
    }

    pub fn new_with_codes ( error_code : u8, exception_code : u8 ) -> ReturnBad
    {
        let l_return : ReturnBad = ReturnBad
                                   {
                                       error_code : Some ( error_code ),
                                       exception_code : Some ( exception_code ),
                                       message : get_message_of_exception_code ( exception_code )
                                   };

        return l_return;
    }

    pub fn new_with_message ( message : &str ) -> ReturnBad
    {
        let l_return : ReturnBad = ReturnBad
                                   {
                                       error_code : None,
                                       exception_code : None,
                                       message : Some ( message.to_owned ( ) )
                                   };

        return l_return;
    }

}


fn get_message_of_exception_code ( code : u8 ) -> Option<String>
{
    let l_return : Option<String>;
 
    let l_message : String =
        match code
        {
            0x01 => "Exception Code 01 - ILLEGAL FUNCTION".to_string ( ),
            0x02 => "Exception Code 02 - ILLEGAL DATA ADDRESS".to_string ( ),
            0x03 => "Exception Code 03 - ILLEGAL DATA VALUE".to_string ( ),
            0x04 => "Exception Code 04 - SERVER DEVICE FAILURE".to_string ( ),
            _ => "Unknown Exception Code".to_string ( )
        };

    l_return = Some ( l_message );

    return l_return;
}


pub struct ReturnGood<T>
{
    data : Option<Vec<T>>,

    duration_in_milliseconds : Option<u64>
}


impl<T> ReturnGood<T>
{

    pub fn get_data ( &mut self ) -> Vec<T>
    {
        let l_return : Vec<T>;

        if self.data.is_some ( )
        {
            l_return = self.data.take ( ).unwrap ( );
        }
        else
        {
            l_return = vec![];
        }

        return l_return;
    }

    pub fn get_duration_in_milliseconds ( &mut self ) -> u64
    {
        let l_return : u64;

        if self.duration_in_milliseconds.is_some ( )
        {
            l_return = self.duration_in_milliseconds.take ( ).unwrap ( );
        }
        else
        {
            l_return = 0;
        }

        return l_return;
    }

    pub fn new ( data : Vec<T>, duration_in_milliseconds : u64 ) -> ReturnGood<T>
    {
        let l_return : ReturnGood<T> = ReturnGood
                                       {
                                           data : Some ( data ),
                                           duration_in_milliseconds : Some ( duration_in_milliseconds )
                                       };
        
        return l_return;
    }

}
