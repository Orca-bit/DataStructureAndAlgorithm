struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut xor = (0..=(nums.len()) as i32).fold(0, |a, b| a ^ b);
        nums.into_iter().fold(xor, |a, b| a ^ b)
    }
}

#[test]
fn test() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(Solution::missing_number(nums), 8);
}
