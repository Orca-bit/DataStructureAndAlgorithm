use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left_max = vec![height[0]; height.len()];
        for i in 1..height.len() {
            left_max[i] = max(left_max[i - 1], height[i]);
        }
        let mut right_max = vec![*height.last().unwrap(); height.len()];
        for i in (0..(height.len() - 1)).rev() {
            right_max[i] = max(right_max[i + 1], height[i]);
        }
        let mut res = 0;
        for i in 1..height.len() - 1 {
            res += max(0, min(left_max[i - 1], right_max[i + 1]) - height[i]);
        }
        res
    }

    pub fn trap_1(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left_max = height[0];
        let mut right_max = *height.last().unwrap();
        let mut left = 1;
        let mut right = height.len() - 1;
        let mut res = 0;
        while left <= right {
            if left_max < right_max {
                res += max(0, left_max - height[left]);
                left_max = max(left_max, height[left]);
                left += 1;
            } else {
                res += max(0, right_max - height[right]);
                right_max = max(right_max, height[right]);
                right -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let v = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(Solution::trap(v), 6);
    let v = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(Solution::trap_1(v), 6);
}
