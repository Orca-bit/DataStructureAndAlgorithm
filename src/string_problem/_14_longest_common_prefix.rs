struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let ss = strs
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let n = ss.iter().map(|s| s.len()).min().unwrap();
        let mut res = "".to_string();
        for i in 0..n {
            let c = ss[0][i];
            if ss.iter().all(|s| s[i] == c) {
                res.push(c);
            } else {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let ss: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!(Solution::longest_common_prefix(ss), String::from("fl"));
}
