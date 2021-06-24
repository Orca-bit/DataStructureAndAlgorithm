use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_palindrome_subsequence(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![1; n]; n];
        for i in 0..n - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = 2;
            }
        }
        if n > 2 {
            for i in (0..n - 2).rev() {
                for j in i + 2..n {
                    dp[i][j] = max(dp[i + 1][j - 1], max(dp[i + 1][j], dp[i][j - 1]));
                    if s[i] == s[j] {
                        dp[i][j] = max(dp[i][j], dp[i + 1][j - 1] + 2);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome_subsequence("a".to_string()), 1);
}
