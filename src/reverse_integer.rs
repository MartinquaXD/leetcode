//https://leetcode.com/problems/reverse-integer/

pub fn reverse(x: i32) -> i32 {
    let mut rev: i32 = 0;
    let mut remainder = x.abs();
    while remainder > 0 {
        let digit = remainder % 10;
        match rev.checked_mul(10).and_then(|res| res.checked_add(digit)) {
            Some(res) => rev = res,
            None => return 0
        }
        remainder /= 10;
    };

    if x < 0 {
        -rev
    } else {
        rev
    }
}