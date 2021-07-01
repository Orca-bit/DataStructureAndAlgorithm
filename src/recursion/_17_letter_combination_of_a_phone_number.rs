struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let map = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let digits = digits.bytes().collect::<Vec<_>>();
        let mut res = vec![];
        let mut path = String::new();
        Self::process(&digits, 0, &map, &mut res, &mut path);
        res
    }

    fn process(
        digits: &[u8],
        index: usize,
        map: &Vec<Vec<char>>,
        res: &mut Vec<String>,
        path: &mut String,
    ) {
        if path.len() == digits.len() {
            res.push(path.clone().to_string());
            return;
        }
        let candidates = &map[(digits[index] - b'2') as usize];
        for &ch in candidates {
            path.push(ch);
            Self::process(digits, index + 1, map, res, path);
            path.pop();
        }
    }
}

#[test]
fn test() {
    let digits = "23".to_string();
    let res: Vec<String> = vec![
        "ad".to_string(),
        "ae".to_string(),
        "af".to_string(),
        "bd".to_string(),
        "be".to_string(),
        "bf".to_string(),
        "cd".to_string(),
        "ce".to_string(),
        "cf".to_string(),
    ];
    assert_eq!(Solution::letter_combinations(digits), res);
}
