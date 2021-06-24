use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.trim().chars().collect::<Vec<_>>();
        let text2 = text2.trim().chars().collect::<Vec<_>>();
        let n = text1.len();
        let m = text2.len();
        if n == 0 || m == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; m]; n];
        if text1[0] == text2[0] {
            dp[0][0] = 1;
        }
        for j in 1..m {
            if text2[j] == text1[0] || dp[0][j - 1] == 1 {
                dp[0][j] = 1;
            }
        }
        for i in 1..n {
            if text1[i] == text2[0] || dp[i - 1][0] == 1 {
                dp[i][0] = 1;
            }
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = max(dp[i - 1][j - 1], max(dp[i - 1][j], dp[i][j - 1]));
                if text1[i] == text2[j] {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + 1);
                }
            }
        }
        dp[n - 1][m - 1]
    }
    pub fn longest_common_subsequence_1(text1: String, text2: String) -> i32 {
        let text1 = text1.trim().chars().collect::<Vec<_>>();
        let text2 = text2.trim().chars().collect::<Vec<_>>();
        let n = text1.len();
        let m = text2.len();
        if n == 0 || m == 0 {
            return 0;
        }
        let mut dp = vec![0; m];
        if text1[0] == text2[0] {
            dp[0] = 1;
        }
        for j in 1..m {
            if text2[j] == text1[0] || dp[j - 1] == 1 {
                dp[j] = 1;
            }
        }
        let mut old = 0;
        for i in 1..n {
            for j in 0..m {
                if j == 0 {
                    old = dp[j];
                    if text1[i] == text2[j] || dp[j] == 1 {
                        dp[j] = 1;
                    }
                } else {
                    let temp = dp[j];
                    dp[j] = max(old, max(dp[j], dp[j - 1]));
                    if text1[i] == text2[j] {
                        dp[j] = max(dp[j], old + 1);
                    }
                    old = temp;
                }
            }
        }
        dp[m - 1]
    }
}

#[test]
fn test() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    let text1 = "abc".to_string();
    let text2 = "abc".to_string();
    assert_eq!(Solution::longest_common_subsequence_1(text1, text2), 3);
    let text1 = "abc".to_string();
    let text2 = "def".to_string();
    assert_eq!(Solution::longest_common_subsequence_1(text1, text2), 0);
}
