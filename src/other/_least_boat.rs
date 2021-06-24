use std::cmp::max;

fn get_least_boats(mut nums: Vec<i32>, limit: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    nums.sort_unstable();
    let n = nums.len();
    if nums[0] > limit / 2 {
        return n as i32;
    }
    if *nums.last().unwrap() > limit {
        panic!("Someone is too fat");
    }
    if *nums.last().unwrap() <= limit / 2 {
        return (n + 1) as i32 / 2;
    }
    let mut less_r = -1;
    match nums.binary_search(&(limit / 2)) {
        Ok(mut index) => {
            while index < n - 1 && nums[index + 1] == nums[index] {
                index += 1;
                less_r = index as i32;
            }
        }
        Err(index) => {
            less_r = index as i32 - 1;
        }
    }
    let mut l = less_r;
    let mut r = less_r + 1;
    let mut left_unused = 0;
    while l >= 0 {
        let mut solved = 0;
        while r < n as i32 && nums[l as usize] + nums[r as usize] <= limit {
            r += 1;
            solved += 1;
        }
        if solved == 0 {
            left_unused += 1;
            l -= 1;
        } else {
            l = max(-1, l - solved);
        }
    }
    let less_used = less_r + 1 - left_unused;
    let right_unsolved = n as i32 - less_r - 1 - less_used;
    less_used + ((left_unused + 1) >> 1) + right_unsolved
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 3, 3, 2, 2, 3, 3, 4, 4, 4, 5, 4, 3];
    assert_eq!(get_least_boats(nums, 6), 8);
}
