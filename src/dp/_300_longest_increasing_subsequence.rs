struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        for &num in nums.iter() {
            if let Err(index) = dp.binary_search(&num) {
                if index == dp.len() {
                    dp.push(num);
                } else {
                    dp[index] = num;
                }
            }
        }
        dp.len() as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
    let res = 6;
    assert_eq!(Solution::length_of_lis(nums), res);
    let nums = vec![0, 1, 0, 3, 2, 3];
    let res = 4;
    assert_eq!(Solution::length_of_lis(nums), res);
}