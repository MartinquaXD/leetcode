//https://leetcode.com/problems/container-with-most-water

//TODO understand proof

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut water = 0;

    while left < right {
        let h_left = height[left];
        let h_right = height[right];
        let distance = right as i32 - left as i32;

        let this_water = distance * std::cmp::min(h_left, h_right);
        water = std::cmp::max(water, this_water);

        if h_left == h_right {
            left += 1;
            right -= 1;
        } else if h_left < h_right {
            left += 1;
        } else {
            right -= 1;
        }
    }

    water
}