use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        if s.is_empty() || s.len() == 1 {
            return 0;
        }
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n - 1 {
            if s[i] != s[i + 1] {
                dp[i][i + 1] = 1;
            }
        }
        if n > 2 {
            for i in (0..n - 2).rev() {
                for j in i + 2..n {
                    dp[i][j] = min(dp[i + 1][j], dp[i][j - 1]) + 1;
                    if s[i] == s[j] {
                        dp[i][j] = min(dp[i][j], dp[i + 1][j - 1]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
    assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
    assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
}
