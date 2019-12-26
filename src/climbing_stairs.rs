//https://leetcode.com/problems/climbing-stairs/

/*
1 => 1
2 => 2
3 => 3     3  - 1 +  rest, 3 - 2 + rest
4 => 5     3 steps, 2 steps =>
5 => 5 + 3
*/

fn climb(n: i32, lookup: &mut Vec<i32>) -> i32 {
    match lookup.get(n as usize) {
        Some(res) => res.clone(),
        None => {
            let small = climb(n - 1, lookup);
            let big = climb(n - 2, lookup);
            let total = small + big;
            lookup.push(total);
            total
        }
    }
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut lookup = vec![1, 1, 2];
    climb(n, &mut lookup)
}