struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.bytes().collect::<Vec<_>>();
        let mut map = vec![-1; 256];
        let mut res = 0;
        let mut pre = -1;
        for (index, &ch) in s.iter().enumerate() {
            pre = pre.max(map[ch as usize]);
            let cur = index as i32 - pre;
            res = res.max(cur);
            map[ch as usize] = index as i32;
        }
        res
    }
}

#[test]
fn test() {
    let s = " ".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "abba".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    let s = "bbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}