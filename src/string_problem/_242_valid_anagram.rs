struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut v = vec![0; 26];
        for c in s.bytes() {
            v[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            v[(c - b'a') as usize] -= 1;
            if v[(c - b'a') as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
