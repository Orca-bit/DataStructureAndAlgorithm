struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("unrecognized char!"),
            })
            .collect::<Vec<_>>();
        let mut res = 0;
        for i in 0..s.len() - 1 {
            if s[i] < s[i + 1] {
                res -= s[i];
            } else {
                res += s[i];
            }
        }
        res += *s.last().unwrap();
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
}
