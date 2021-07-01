struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            if ch == '(' || ch == '[' || ch == '{' {
                stack.push(ch);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let last = stack.pop().unwrap();
                if (ch == ')' && last != '(')
                    || (ch == ']' && last != '[')
                    || (ch == '}' && last != '{')
                {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}


#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}
