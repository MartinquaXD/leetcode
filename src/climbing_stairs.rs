//https://leetcode.com/problems/climbing-stairs/

//TODO refactor to iterative loop and only two variables instead of lookup vec

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