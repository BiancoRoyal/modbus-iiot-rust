

use std::time::Duration;


#[test]
fn test_compute_milliseconds ( )
{
    let l_test_1_data : Duration = Duration::new ( 0, 2000000 );

    let l_result_1_data = compute_milliseconds ( &l_test_1_data );

    assert_eq! ( l_result_1_data, 2 );
}

pub fn compute_milliseconds ( duration : &Duration ) -> u64
{
    return ( duration.as_secs ( ) * 1000 ) + ( ( duration.subsec_nanos ( ) / 1000000 ) as u64 );
}