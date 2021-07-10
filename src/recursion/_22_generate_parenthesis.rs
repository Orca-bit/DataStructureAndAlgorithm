struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut path = vec![];
        Self::process(&mut res, &mut path, 0, n, (n << 1) as usize);
        res
    }

    fn process(
        res: &mut Vec<String>,
        path: &mut Vec<char>,
        left_sum: i32,
        left_rest: i32,
        total: usize,
    ) {
        if path.len() == total {
            res.push(path.iter().collect());
            return;
        }
        if left_rest > 0 {
            path.push('(');
            Self::process(res, path, left_sum + 1, left_rest - 1, total);
            path.pop();
        }
        if left_sum > 0 {
            path.push(')');
            Self::process(res, path, left_sum - 1, left_rest, total);
            path.pop();
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
