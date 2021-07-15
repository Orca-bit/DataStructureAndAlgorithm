struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut res = 0;
        for c in column_title.bytes() {
            res = res * 26 + (c - b'A') as i32 + 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::title_to_number("A".to_string()), 1);
    assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
}
