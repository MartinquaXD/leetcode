mod add_tow_numbers;
mod palindrome_number;
mod reverse_integer;
mod jewels_and_stones;
mod hamming_distance;
mod zigzag_conversion;
mod container_with_most_water;
mod letter_combinations_of_a_phone_number;
mod divide_two_integers;
mod climbing_stairs;

fn main() {
    let res = climbing_stairs::climb_stairs(5);
//    let res = divide_two_integers::divide(-12, -1);
    println!("{:#?}", res);
//    println!("{}", zigzag_conversion::convert("aA".to_string(), 4));
    println!("Hello, world!");
}
