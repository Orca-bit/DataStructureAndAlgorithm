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

    fn dp(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut first_hand_dp = vec![vec![0; n]; n];
        let mut second_hand_dp = vec![vec![0; n]; n];
        for i in 0..n {
            first_hand_dp[i][i] = nums[i];
        }
        for i in (0..(n - 1)).rev() {
            for j in (i + 1)..n {
                first_hand_dp[i][j] = max(
                    nums[i] + second_hand_dp[i + 1][j],
                    nums[j] + second_hand_dp[i][j - 1],
                );
                second_hand_dp[i][j] = min(first_hand_dp[i + 1][j], first_hand_dp[i][j - 1]);
            }
        }
        first_hand_dp[0][n - 1] >= second_hand_dp[0][n - 1]
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 2];
    let res = false;
    assert_eq!(Solution::predict_the_winner(nums), res);
    let nums = vec![1, 5, 233, 7];
    let res = true;
    assert_eq!(Solution::dp(nums), res);
}
