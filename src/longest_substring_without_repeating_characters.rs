//https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

//naive
//pub fn length_of_longest_substring(s: String) -> i32 {
//    let mut longest = 0;
//    for (index, c) in s.chars().enumerate() {
//        let mut chars: HashSet<char> = HashSet::default();
//
//        for c in s.chars().skip(index) {
//            if !chars.insert(c) {
//                break;
//            }
//        }
//
//        longest = std::cmp::max(longest, chars.len() as i32);
//    }
//
//    longest
//}

use std::collections::HashMap;
use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut len = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut res = 0;
    for (index, c) in s.chars().enumerate() {
        if map.contains_key(&c) {
            len = max(len, map[&c] + 1);
        }
        res = max(res, index - len + 1);
        map.insert(c, index);
    }
    return res as i32;
}