struct Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let (mut len_0, mut len_1) = (0, 0);
        let (mut max_0, mut max_1) = (0, 0);
        for c in s.chars() {
            if c == '0' {
                len_1 = 0;
                len_0 += 1;
                max_0 = max_0.max(len_0);
            }
            if c == '1' {
                len_0 = 0;
                len_1 += 1;
                max_1 = max_1.max(len_1);
            }
        }
        max_1 > max_0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
    assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
}
