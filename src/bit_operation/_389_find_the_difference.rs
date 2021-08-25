struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        (s + t.as_str()).bytes().fold(0, |acc, x| acc ^ x) as char
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
