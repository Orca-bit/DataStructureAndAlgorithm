use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::first_hand(&nums, 0, nums.len() - 1) >= Self::second_hand(&nums, 0, nums.len() - 1)
    }

    fn first_hand(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right {
            return nums[left];
        }
        max(
            nums[left] + Self::second_hand(nums, left + 1, right),
            nums[right] + Self::second_hand(nums, left, right - 1),
        )
    }

    fn second_hand(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right {
            return 0;
        }
        min(
            Self::first_hand(nums, left + 1, right),
            Self::first_hand(nums, left, right - 1),
        )
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 2];
    let res = false;
    assert_eq!(Solution::predict_the_winner(nums), res);
    let nums = vec![1, 5, 233, 7];
    let res = true;
    assert_eq!(Solution::predict_the_winner(nums), res);
}
