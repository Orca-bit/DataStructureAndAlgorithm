struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if s.is_empty() || s.len() == 1 {
            return 0;
        }
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut is_palindrome = vec![vec![true; n]; n];
        for i in 0..n - 1 {
            if s[i] != s[i + 1] {
                is_palindrome[i][i + 1] = false;
            }
        }
        if n > 2 {
            for i in (0..n - 2).rev() {
                for j in i + 2..n {
                    if !is_palindrome[i + 1][j - 1] || s[i] != s[j] {
                        is_palindrome[i][j] = false;
                    }
                }
            }
        }
        let mut dp = vec![None; n + 1];
        Self::process(&s, 0, &is_palindrome, &mut dp)
    }

    fn process(
        s: &Vec<char>,
        index: usize,
        is_palindrome: &Vec<Vec<bool>>,
        dp: &mut Vec<Option<i32>>,
    ) -> i32 {
        if let Some(record) = dp[index] {
            return record;
        }
        if index == s.len() {
            dp[index] = Some(-1);
            return -1;
        }
        let mut res = i32::MAX;
        for end in index..s.len() {
            if is_palindrome[index][end] {
                res = res.min(Self::process(s, end + 1, is_palindrome, dp) + 1);
            }
        }
        dp[index] = Some(res);
        res
    }

    fn dp(s: String) -> i32 {
        if s.is_empty() || s.len() == 1 {
            return 0;
        }
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[n] = -1;
        let mut is_palindrome = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if s[i] == s[j] && (j - i < 2 || is_palindrome[i + 1][j - 1]) {
                    is_palindrome[i][j] = true;
                    dp[i] = dp[i].min(dp[j + 1] + 1);
                }
            }
        }
        dp[0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_cut("aa".to_string()), 0);
    assert_eq!(Solution::dp("aa".to_string()), 0);
}
