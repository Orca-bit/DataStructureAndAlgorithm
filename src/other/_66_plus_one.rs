struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        for i in (0..n).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut res = vec![0; n + 1];
        res[0] = 1;
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0])
}