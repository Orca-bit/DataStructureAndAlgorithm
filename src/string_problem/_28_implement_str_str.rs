struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.is_empty() || haystack.len() < needle.len() {
            return -1;
        }
        let s = haystack.chars().collect::<Vec<_>>();
        let m = needle.chars().collect::<Vec<_>>();
        let next = Self::next_array(&m);
        let mut si = 0;
        let mut mi = 0;
        while si < s.len() && mi < m.len() {
            if s[si] == m[mi] {
                si += 1;
                mi += 1;
            } else if next[mi] != -1 {
                mi = next[mi] as usize;
            } else {
                si += 1;
            }
        }
        if mi == m.len() {
            (si - mi) as i32
        } else {
            -1
        }
    }

    fn next_array(s: &[char]) -> Vec<i32> {
        assert!(!s.is_empty());
        if s.len() == 1 {
            return vec![-1];
        }
        let mut res = vec![0; s.len()];
        res[0] = -1;
        let mut pos = 2;
        let mut cn = 0;
        while pos < res.len() {
            if s[pos - 1] == s[cn] {
                res[pos] = cn as i32 + 1;
                cn += 1;
                pos += 1;
            } else if res[cn] != -1 {
                cn = res[cn] as usize;
            } else {
                res[pos] = 0;
                pos += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    );
}
