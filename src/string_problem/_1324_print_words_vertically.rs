struct Solution;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let v = s.split_whitespace().collect::<Vec<_>>();
        let mut res = vec![];
        for (i, s) in v.into_iter().enumerate() {
            for (j, c) in s.char_indices() {
                if res.len() == j {
                    res.push("".to_string());
                }
                while res[j].len() < i {
                    res[j].push(' ');
                }
                res[j].push(c);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "HOW ARE YOU".to_string();
    let res = vec!["HAY".to_string(), "ORO".to_string(), "WEU".to_string()];
    assert_eq!(Solution::print_vertically(s), res);
    let s = "TO BE OR NOT TO BE".to_string();
    let res = vec![
        "TBONTB".to_string(),
        "OEROOE".to_string(),
        "   T".to_string(),
    ];
    assert_eq!(Solution::print_vertically(s), res);
    let s = "CONTEST IS COMING".to_string();
    let res = vec![
        "CIC".to_string(),
        "OSO".to_string(),
        "N M".to_string(),
        "T I".to_string(),
        "E N".to_string(),
        "S G".to_string(),
        "T".to_string(),
    ];
    assert_eq!(Solution::print_vertically(s), res);
}
