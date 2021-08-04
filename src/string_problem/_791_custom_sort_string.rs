use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map = HashMap::new();
        for (i, c) in order.char_indices() {
            *map.entry(c).or_default() = i;
        }
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_unstable_by_key(|c| map.get(c).unwrap_or(&26));
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "cba".to_string();
    let t = "abcd".to_string();
    let res = "cbad".to_string();
    assert_eq!(Solution::custom_sort_string(s, t), res);
}
