struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut res = 0;
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0; s.len()];
        for i in 1..s.len() {
            if s[i] == ')' {
                let pre = i as i32 - 1 - dp[i - 1];
                if pre >= 0 && s[pre as usize] == '(' {
                    dp[i] = dp[i - 1] + 2 + if pre > 0 { dp[pre as usize - 1] } else { 0 };
                    res = res.max(dp[i]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "(()".to_string();
    let res = 2;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
    let s = ")()())".to_string();
    let res = 4;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
    let s = "()(())".to_string();
    let res = 6;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
}