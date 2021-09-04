struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stk = vec![];
        for c in s.chars() {
            if stk.last().is_some() && *stk.last().unwrap() == c {
                stk.pop();
                continue;
            }
            stk.push(c);
        }
        stk.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "abbaca".to_string();
    let t = "ca".to_string();
    assert_eq!(Solution::remove_duplicates(s), t);
}
