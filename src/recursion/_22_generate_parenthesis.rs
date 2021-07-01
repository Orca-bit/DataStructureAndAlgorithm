struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut path = vec![' '; (n << 1) as usize];
        Self::process(&mut res, &mut path, 0, 0, n);
        res
    }

    fn process(
        res: &mut Vec<String>,
        path: &mut Vec<char>,
        index: usize,
        left_sum: i32,
        left_rest: i32,
    ) {
        if index == path.len() {
            res.push(path.iter().collect());
            return;
        }
        if left_rest > 0 {
            path[index] = '(';
            Self::process(res, path, index + 1, left_sum + 1, left_rest - 1);
        }
        if left_sum > 0 {
            path[index] = ')';
            Self::process(res, path, index + 1, left_sum - 1, left_rest);
        }
    }
}

#[test]
fn test() {
    let res: Vec<String> = vec![
        "((()))".to_string(),
        "(()())".to_string(),
        "(())()".to_string(),
        "()(())".to_string(),
        "()()()".to_string(),
    ];
    assert_eq!(Solution::generate_parenthesis(3), res);
}
