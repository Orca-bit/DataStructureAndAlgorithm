use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut res = HashSet::new();
        let mut path = vec![];
        let mut min = usize::MAX;
        let s = s.chars().collect::<Vec<_>>();
        Self::process(0, 0, 0, &mut path, &mut res, &mut min, &s);
        res.into_iter().collect()
    }

    fn process(
        index: usize,
        left: usize,
        remove: usize,
        path: &mut Vec<char>,
        res: &mut HashSet<String>,
        min: &mut usize,
        s: &[char],
    ) {
        if index == s.len() {
            if left != 0 {
                return;
            }
            if remove > *min {
                return;
            }
            if remove < *min {
                *min = remove;
                res.clear();
            }
            res.insert(path.iter().copied().collect());
            return;
        }
        match s[index] {
            '(' => {
                path.push('(');
                Self::process(index + 1, left + 1, remove, path, res, min, s);
                path.pop();
                Self::process(index + 1, left, remove + 1, path, res, min, s);
            }
            ')' => {
                if left > 0 {
                    path.push(')');
                    Self::process(index + 1, left - 1, remove, path, res, min, s);
                    path.pop();
                }
                Self::process(index + 1, left, remove + 1, path, res, min, s);
            }
            _ => {
                path.push(s[index]);
                Self::process(index + 1, left, remove, path, res, min, s);
                path.pop();
            }
        }
    }
}

#[test]
fn test() {
    let s = "()())()".to_string();
    let mut res = vec!["()()()".to_string(), "(())()".to_string()];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = "(a)())()".to_string();
    let mut res = vec!["(a)()()".to_string(), "(a())()".to_string()];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(".to_string();
    let mut res = vec!["".to_string()];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(f".to_string();
    let mut res = vec!["f".to_string()];
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
