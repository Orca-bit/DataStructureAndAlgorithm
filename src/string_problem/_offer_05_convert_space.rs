struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut iter = s.chars();
        let mut res = String::new();
        while let Some(c) = iter.next() {
            if c.is_whitespace() {
                res.push_str("%20");
            } else {
                res.push(c);
            }
        }
        res
    }
}

#[test]
fn test() {
    println!("{}", Solution::replace_space("We are happy".to_string()));
}