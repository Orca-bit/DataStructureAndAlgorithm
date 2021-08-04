struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let (steps, arr_len) = (steps as usize, arr_len as usize);
        let m = (steps + 1).min(arr_len);
        let n = steps;
        let mut dp = vec![0; m];
        dp[0] = 1;
        for _ in 1..n {
            let mut old_1 = 0;
            let mut old = dp[0];
            for j in 0..m {
                // dp[i][j] = dp[i - 1][j];
                let tmp = dp[j];
                old = tmp;
                if j > 0 {
                    // dp[i][j] += dp[i - 1][j - 1];
                    // dp[i][j] %= MOD;
                    dp[j] += old_1;
                    dp[j] %= MOD;
                }
                if j < m - 1 {
                    // dp[i][j] += dp[i - 1][j + 1];
                    // dp[i][j] %= MOD;
                    dp[j] += dp[j + 1];
                    dp[j] %= MOD;
                }
                old_1 = old;
            }
        }
        (dp[0] + if m > 1 { dp[1] } else { 0 }) % MOD
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_ways(3, 3), 4);
    assert_eq!(Solution::num_ways(2, 4), 2);
    assert_eq!(Solution::num_ways(4, 2), 8);
    assert_eq!(Solution::num_ways(1, 1), 1);
}
