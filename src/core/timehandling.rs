

use std::time::{Duration, SystemTime};

//	===============================================================================================

pub struct Timestamp
{
    time : SystemTime
}

impl Timestamp
{
    pub fn new () -> Timestamp
    {
        return Timestamp { time : SystemTime::now () };
    }

    pub fn elapsed_milliseconds ( &self ) -> u64
    {
        let elapsed_time : Duration = self.time.elapsed ().unwrap ();
        return compute_milliseconds ( &elapsed_time );
    }

    pub fn elapsed_time ( &self ) -> Duration
    {
        return self.time.elapsed ().unwrap ();
    }
}

//	===============================================================================================

#[test]
fn test_compute_milliseconds ( )
{
    let test_data : Duration = Duration::new ( 0, 
                                               2000000 );

    let result_data = compute_milliseconds ( &test_data );
    assert_eq! ( result_data, 2 );
}

fn compute_milliseconds ( duration : &Duration ) -> u64
{
    return ( duration.as_secs () * 1000 ) + ( ( duration.subsec_nanos () / 1000000 ) as u64 );
}
