struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let s = text.as_bytes();
        let n = s.len();
        let mut table = vec![0; 26];
        let mut res = 0;
        for &c in s {
            let index = (c - b'a') as usize;
            table[index] += 1;
            res = res.max(table[index]);
        }
        if res <= 1 {
            return res;
        }
        res = 1;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            let mut k = j + 1;
            while k < n && s[k] == s[i] {
                k += 1;
            }
            let mut candidate = (k - i - 1) as i32;
            if candidate < table[(s[i] - b'a') as usize] {
                candidate += 1;
            }
            res = res.max(candidate);
            i = j;
        }
        res
    }
}

#[test]
fn test() {
    let text = "ababa".to_string();
    let res = 3;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaabaaa".to_string();
    let res = 6;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaabbaaa".to_string();
    let res = 4;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaaaa".to_string();
    let res = 5;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "abcdef".to_string();
    let res = 1;
    assert_eq!(Solution::max_rep_opt1(text), res);
    assert_eq!(Solution::max_rep_opt1("bbababaaaa".to_string()), 6);
}
