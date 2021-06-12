struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![-1; n as usize + 1];
        Self::process(n as usize, &mut dp)
    }

    fn process(n: usize, dp: &mut [i32]) -> i32 {
        if dp[n] != -1 {
            return dp[n];
        }
        if n == 0 || n == 1 {
            dp[n] = 1;
        } else {
            dp[n] = 0;
            for i in 1..=n {
                let left = Self::process(i - 1, dp);
                let right= Self::process(n - i, dp);
                dp[n] += left * right;
            }
        }
        dp[n]
    }

    fn dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 5;
    assert_eq!(Solution::num_trees(n), res);
    let n = 3;
    let res = 5;
    assert_eq!(Solution::dp(n), res);
}