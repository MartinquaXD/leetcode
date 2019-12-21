//https://leetcode.com/problems/jewels-and-stones/

use std::collections::hash_set::HashSet;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let jewels = j.chars().collect::<HashSet<_>>();
    s.chars().fold(0, |acc, s|
        if jewels.contains(&s) {
            acc + 1
        } else {
            acc
        }
    )
}