struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut zeroes = vec![0; n + 1];
        let mut ones = vec![0; n + 1];
        for i in 0..n {
            zeroes[i + 1] = zeroes[i] + if s[i] == '0' { 1 } else { 0 };
        }
        for i in (0..n).rev() {
            ones[i] = ones[i + 1] + if s[i] == '1' { 1 } else { 0 };
        }
        zeroes[1..n]
            .into_iter()
            .zip(ones[1..n].into_iter())
            .fold(0, |a, b| a.max(b.0 + b.1))
    }
}

#[test]
fn test() {
    let s = "011101".to_string();
    let res = 5;
    assert_eq!(Solution::max_score(s), res);
    let s = "00111".to_string();
    let res = 5;
    assert_eq!(Solution::max_score(s), res);
    let s = "1111".to_string();
    let res = 3;
    assert_eq!(Solution::max_score(s), res);
}
