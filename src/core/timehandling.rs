

use std::time::Duration;

//	===============================================================================================

#[test]
fn test_compute_milliseconds ( )
{
    let test_data : Duration = Duration::new ( 0, 
                                               2000000 );

    let result_data = compute_milliseconds ( &test_data );
    assert_eq! ( result_data, 2 );
}

pub fn compute_milliseconds ( duration : &Duration ) -> u64
{
    return ( duration.as_secs () * 1000 ) + ( ( duration.subsec_nanos () / 1000000 ) as u64 );
}
