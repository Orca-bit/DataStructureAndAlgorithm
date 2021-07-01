use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return Vec::new();
        }
        let mut map: HashMap<_, Vec<String>> = HashMap::new();
        for s in strs {
            let mut tmp = s.chars().collect::<Vec<_>>();
            tmp.sort_unstable();
            map.entry(tmp).or_default().push(s.clone());
        }
        let mut res = vec![];
        for list in map.values() {
            res.push(list.clone());
        }
        res
    }
}

#[test]
fn test() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let res = vec![
        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        vec!["bat".to_string()],
        vec!["nat".to_string(), "tan".to_string()],
    ];
    assert_eq!(Solution::group_anagrams(strs), res);
}
