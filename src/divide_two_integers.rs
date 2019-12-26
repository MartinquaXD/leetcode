//https://leetcode.com/problems/divide-two-integers/

//TODO fix timeout shit

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let result_is_positive = (dividend < 0) == (divisor < 0);



    let (mut dividend, overflowed) = dividend.overflowing_abs();
    if !overflowed {
        dividend = -dividend;
    }

    let (mut divisor, overflowed) = divisor.overflowing_abs();
    if !overflowed {
        divisor = -divisor;
    }

    let mut result: i32 = 0;
    while dividend <= divisor {
        match dividend.checked_sub(divisor){
            Some(res) => dividend = res,
            None => return i32::max_value()
        }
        match result.checked_add(1) {
            Some(res) => result = res,
            None => return i32::max_value()
        }
    };

    if result_is_positive {
        result
    } else {
        -result
    }
}