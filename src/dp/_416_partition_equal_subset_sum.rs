struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total & 1 != 0 {
            return false;
        }
        let n = nums.len();
        let target = (total >> 1) as usize;
        let mut dp = vec![vec![false; target + 1]; n];
        for i in 0..n {
            dp[i][0] = true;
        }
        if nums[0] <= target as i32 {
            dp[0][nums[0] as usize] = true;
        }
        for i in 1..n {
            for j in 1..=target {
                dp[i][j] = dp[i - 1][j];
                if j as i32 - nums[i] >= 0 {
                    dp[i][j] = dp[i][j] || dp[i - 1][j - nums[i] as usize];
                }
            }
            if dp[i][target] == true {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 11, 5];
    let res = true;
    assert_eq!(Solution::can_partition(nums), res);
    let nums = vec![1, 2, 5];
    let res = false;
    assert_eq!(Solution::can_partition(nums), res);
}
