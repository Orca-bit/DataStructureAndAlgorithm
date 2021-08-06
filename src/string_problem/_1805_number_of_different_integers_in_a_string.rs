use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let v = word
            .chars()
            .map(|c| if !c.is_ascii_digit() { ' ' } else { c })
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.chars().rev().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut hs = HashSet::new();
        for mut x in v {
            while let Some(c) = x.pop() {
                if c == '0' && x.last() == Some(&'0') {
                    continue;
                } else {
                    if c != '0' {
                        x.push(c);
                    }
                    break;
                }
            }
            hs.insert(x);
        }
        hs.len() as i32
    }
}

#[test]
fn test() {
    let word = "a123bc34d8ef34".to_string();
    let res = 3;
    assert_eq!(Solution::num_different_integers(word), res);
    let word = "leet1234code234".to_string();
    let res = 2;
    assert_eq!(Solution::num_different_integers(word), res);
    let word = "a1b01c001".to_string();
    let res = 1;
    assert_eq!(Solution::num_different_integers(word), res);
}
