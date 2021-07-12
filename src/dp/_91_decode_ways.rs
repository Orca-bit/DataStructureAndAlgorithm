struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.bytes().collect::<Vec<_>>();
        Self::process(&s, 0)
    }

    fn process(s: &[u8], index: usize) -> i32 {
        if index == s.len() {
            return 1;
        }
        if s[index] == b'0' {
            return 0;
        }
        let mut res = Self::process(s, index + 1);
        if index + 1 < s.len() && (s[index] - b'0') * 10 + (s[index + 1] - b'0') < 27 {
            res += Self::process(s, index + 2);
        }
        res
    }

    fn dp(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.bytes().collect::<Vec<_>>();
        let n = s.len();
        let mut cur = if s[n - 1] == b'0' { 0 } else { 1 };
        let mut next = 1;
        for i in (0..n - 1).rev() {
            if s[i] == b'0' {
                next = cur;
                cur = 0;
            } else {
                let tmp = cur;
                if (s[i] - b'0') * 10 + (s[i + 1] - b'0') < 27 {
                    cur += next;
                }
                next = tmp;
            }
        }
        cur
    }
}

#[test]
fn test() {
    let s = "12".to_string();
    assert_eq!(Solution::num_decodings(s), 2);
    let s = "226".to_string();
    assert_eq!(Solution::num_decodings(s), 3);
    let s = "11106".to_string();
    assert_eq!(Solution::num_decodings(s), 2);
    let s = "12".to_string();
    assert_eq!(Solution::dp(s), 2);
    let s = "226".to_string();
    assert_eq!(Solution::dp(s), 3);
    let s = "11106".to_string();
    assert_eq!(Solution::dp(s), 2);
}
