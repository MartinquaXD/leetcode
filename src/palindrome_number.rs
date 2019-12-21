//https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(x: i32) -> bool {

    //negative numbers and numbers with 0 as the last digit can't be palindromes
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    };

    let mut remainder = x;
    let mut reverted = 0;
    while remainder > 0 {
        let digit = remainder % 10;
        reverted = reverted * 10 + digit;
        remainder /= 10;
    };

    x == reverted
}