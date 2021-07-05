use std::cmp::max;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        let mut help = vec![1; n + 2];
        for i in 1..=n {
            help[i] = nums[i - 1];
        }
        let mut dp = vec![vec![None; n + 1]; n + 1];
        Self::process(&help, 1, n, &mut dp)
    }

    fn process(nums: &[i32], left: usize, right: usize, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if let Some(value) = dp[left][right] {
            return value;
        }
        let mut res = i32::MIN;
        if left == right {
            res = nums[left - 1] * nums[left] * nums[right + 1];
        } else {
            res = max(
                Self::process(nums, left + 1, right, dp)
                    + nums[left - 1] * nums[left] * nums[right + 1],
                Self::process(nums, left, right - 1, dp)
                    + nums[left - 1] * nums[right] * nums[right + 1],
            );
            for i in left + 1..right {
                res = res.max(
                    Self::process(nums, left, i - 1, dp)
                        + Self::process(nums, i + 1, right, dp)
                        + nums[left - 1] * nums[i] * nums[right + 1],
                );
            }
        }
        dp[left][right] = Some(res);
        res
    }

    fn dp(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        let mut nums = nums.into_iter().collect::<VecDeque<_>>();
        nums.push_back(1);
        nums.push_front(1);
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 1..=n {
            dp[i][i] = nums[i - 1] * nums[i] * nums[i + 1];
        }
        for i in (1..n).rev() {
            for j in i + 1..=n {
                dp[i][j] = (dp[i + 1][j] + nums[i - 1] * nums[i] * nums[j + 1])
                    .max(dp[i][j - 1] + nums[i - 1] * nums[j] * nums[j + 1]);
                for k in i + 1..j {
                    dp[i][j] = dp[i][j]
                        .max(dp[i][k - 1] + dp[k + 1][j] + nums[i - 1] * nums[k] * nums[j + 1]);
                }
            }
        }
        dp[1][n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    assert_eq!(Solution::dp(vec![3, 1, 5, 8]), 167);
}
