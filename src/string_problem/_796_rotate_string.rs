struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.is_empty() && goal.is_empty() {
            return true;
        }
        if s.len() != goal.len() {
            return false;
        }
        let s = s.clone() + &*s;
        match KMP::get_index(&s, &goal) {
            -1 => false,
            _ => true,
        }
    }
}

struct KMP;

impl KMP {
    pub fn get_index(s: &str, m: &str) -> i32 {
        if s.is_empty() || m.is_empty() || m.len() < 1 || s.len() < m.len() {
            return -1;
        }
        let s: Vec<_> = s.chars().collect();
        let m: Vec<_> = m.chars().collect();
        let next = Self::get_next(&m);
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

    fn get_next(m: &[char]) -> Vec<i32> {
        let mut next = vec![0; m.len()];
        next[0] = -1;
        let mut pos = 2;
        let mut cn = 0;
        while pos < next.len() {
            if m[pos - 1] == m[cn] {
                cn += 1;
                next[pos] = cn as i32;
                pos += 1;
            } else if next[cn] > 0 {
                cn = next[cn] as usize;
            } else {
                next[pos] = 0;
                pos += 1;
            }
        }
        next
    }
}

#[test]
fn test() {
    let s = "abcde".to_string();
    let m = "cdeab".to_string();
    assert_eq!(Solution::rotate_string(s, m), true);
    let s = "abcde".to_string();
    let m = "abced".to_string();
    assert_eq!(Solution::rotate_string(s, m), false);
}
