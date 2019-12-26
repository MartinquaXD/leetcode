//https://leetcode.com/problems/jump-game-ii/

//TODO implement greedy bfs

/*
pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut start = 1;
        let mut end = nums[0] as usize;
        let mut cover = end;
        let mut count = 1;
        // println!("start: {}, end: {}", start, end);
        while cover < nums.len() - 1 {
            for i in start..end+1 {
                cover = std::cmp::max(cover, i + nums[i] as usize);
            }
            count += 1;
            start = end + 1;
            end = cover;
            // println!("cover: {}, count: {}", cover, count);
        }
        count
    }*/

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut least_jumps_at_index = vec![None; nums.len()];
    least_jumps_at_index[nums.len() - 1] = Some(0);

    for (i, jumps) in nums.iter().cloned().enumerate().rev().skip(1) {
        match least_jumps_at_index.iter()
            .skip(i + 1)
            .take(jumps as usize)
            .filter(|v| v.is_some())
            .min() {
            Some(Some(val)) => least_jumps_at_index[i] = Some(val + 1),
            _ => least_jumps_at_index[i] = None,
        }
    }

    least_jumps_at_index[0].unwrap()
}