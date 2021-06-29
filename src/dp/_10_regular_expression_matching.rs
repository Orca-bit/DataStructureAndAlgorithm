struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();
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
        if let Some(item) = dp[si][pi] {
            return item;
        }
        if si == 0 && pi == 0 {
            dp[si][pi] = Some(true);
            return true;
        }
        if si != 0 && pi == 0 {
            dp[si][pi] = Some(false);
            return false;
        }
        let res = if si == 0 && pi != 0 {
            if p[pi - 1] == '*' {
                Self::process(s, p, si, pi - 2, dp)
            } else {
                false
            }
        } else {
            if s[si - 1] == p[pi - 1] {
                Self::process(s, p, si - 1, pi - 1, dp)
            } else {
                match p[pi - 1] {
                    '.' => Self::process(s, p, si - 1, pi - 1, dp),
                    '*' => match p[pi - 2] {
                        '*' => false,
                        '.' => {
                            Self::process(s, p, si - 1, pi, dp)
                                || Self::process(s, p, si, pi - 2, dp)
                        }
                        _ => {
                            if s[si - 1] != p[pi - 2] {
                                Self::process(s, p, si, pi - 2, dp)
                            } else {
                                Self::process(s, p, si - 1, pi, dp)
                                    || Self::process(s, p, si, pi - 2, dp)
                            }
                        }
                    },
                    _ => false,
                }
            }
        };
        dp[si][pi] = Some(res);
        res
    }
}

#[test]
fn test() {
    let s = "aa".to_string();
    let p = "a".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "aa".to_string();
    let p = "a*".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "ab".to_string();
    let p = ".*".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "aab".to_string();
    let p = "c*a*b".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "mississippi".to_string();
    let p = "mis*is*p*.".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
}
