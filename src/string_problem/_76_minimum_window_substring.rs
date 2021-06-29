use std::str::from_utf8;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let s = s.bytes().collect::<Vec<_>>();
        let t = t.bytes().collect::<Vec<_>>();
        let mut map = vec![0; 256];
        for &item in t.iter() {
            map[item as usize] += 1;
        }
        let mut left = 0;
        let mut right = 0;
        let mut all = t.len();
        let mut res = (usize::MAX, "");
        while right < s.len() {
            map[s[right] as usize] -= 1;
            if map[s[right] as usize] >= 0 {
                all -= 1;
            }
            if all == 0 {
                while map[s[left] as usize] < 0 {
                    map[s[left] as usize] += 1;
                    left += 1;
                }
                let new_value = right - left + 1;
                if new_value < res.0 {
                    res = (new_value, from_utf8(&s[left..=right]).unwrap());
                }
                map[s[left] as usize] += 1;
                left += 1;
                all += 1;
            }
            right += 1;
        }
        res.1.to_string()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
}
