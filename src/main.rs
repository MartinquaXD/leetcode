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
mod longest_substring_without_repeating_characters;
mod jump_game_2;

fn main() {
    let res = jump_game_2::jump(vec![2,3,0,1,4]);
//    let res = divide_two_integers::divide(-12, -1);
    println!("{:#?}", res);
//    println!("{}", zigzag_conversion::convert("aA".to_string(), 4));
    println!("Hello, world!");
}
