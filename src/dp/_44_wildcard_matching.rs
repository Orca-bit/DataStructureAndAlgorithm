struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();
        let mut dp = vec![vec![None; p.len() + 1]; s.len() + 1];
        Self::process(&s, &p, s.len(), p.len(), &mut dp)
    }

    fn process(
        s: &[char],
        p: &[char],
        si: usize,
        pi: usize,
        dp: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        // 当前匹配到的位置是index - 1
        if let Some(item) = dp[si][pi] {
            return item;
        }
        if si == 0 && pi == 0 { // 意味着反向遍历完没被打断
            dp[si][pi] = Some(true);
        } else if si != 0 && pi == 0 { // s没遍历完，p已经结束了，必然不匹配
            dp[si][pi] = Some(false);
        } else {
            if si == 0 && pi != 0 { // s已经遍历完了，p还有，那么只能为 *
                if p[pi - 1] == '*' {
                    dp[si][pi] = Some(Self::process(s, p, si, pi - 1, dp));
                } else {
                    dp[si][pi] = Some(false);
                }
            } else { // 都没遍历完
                if s[si - 1] == p[pi - 1] { //当前字符相同
                    dp[si][pi] = Some(Self::process(s, p, si - 1, pi - 1, dp));
                } else {
                    dp[si][pi] = match p[pi - 1] {
                        '?' => Some(Self::process(s, p, si - 1, pi - 1, dp)), // ? 类似相同情况
                        '*' => Some(    // * 可以不参与匹配，也可以覆盖当前字符
                            Self::process(s, p, si, pi - 1, dp)
                                || Self::process(s, p, si - 1, pi, dp),
                        ),
                        _ => Some(false),
                    };
                }
            }
        }
        dp[si][pi].unwrap()
    }

    // 记忆化递归改dp
    fn dp(s: String, p: String) -> bool {
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for pi in 1..=p.len() {
            dp[0][pi] = if p[pi - 1] == '*' {
                dp[0][pi - 1]
            } else {
                false
            };
        }
        for si in 1..=s.len() {
            for pi in 1..=p.len() {
                dp[si][pi] = if s[si - 1] == p[pi - 1] {
                    dp[si - 1][pi - 1]
                } else {
                    match p[pi - 1] {
                        '?' => dp[si - 1][pi - 1],
                        '*' => dp[si - 1][pi] || dp[si][pi - 1],
                        _ => false,
                    }
                };
            }
        }
        dp[s.len()][p.len()]
    }
}

#[test]
fn test() {
    let s = "aa".to_string();
    let p = "a".to_string();
    let res = false;
    assert_eq!(Solution::dp(s, p), res);
    let s = "aa".to_string();
    let p = "*".to_string();
    let res = true;
    assert_eq!(Solution::dp(s, p), res);
    let s = "cb".to_string();
    let p = "?a".to_string();
    let res = false;
    assert_eq!(Solution::dp(s, p), res);
    let s = "adceb".to_string();
    let p = "*a*b".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "acdcb".to_string();
    let p = "a*c?b".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
}
