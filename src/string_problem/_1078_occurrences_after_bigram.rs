struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let v = text.split_whitespace().collect::<Vec<_>>();
        let n = v.len();
        let mut res = vec![];
        for i in 1..n - 1 {
            if v[i - 1] == first.as_str() && v[i] == second.as_str() {
                res.push(v[i + 1].to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    let text = "alice is a good girl she is a good student".to_string();
    let first = "a".to_string();
    let second = "good".to_string();
    let res: Vec<String> = vec!["girl".to_string(), "student".to_string()];
    assert_eq!(Solution::find_ocurrences(text, first, second), res);
    let text = "we will we will rock you".to_string();
    let first = "we".to_string();
    let second = "will".to_string();
    let res: Vec<String> = vec!["we".to_string(), "rock".to_string()];
    assert_eq!(Solution::find_ocurrences(text, first, second), res);
}
