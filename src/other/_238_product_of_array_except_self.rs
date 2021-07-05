struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        res[0] = nums[0];
        for i in 1..res.len() {
            res[i] = res[i - 1] * nums[i];
        }
        let mut tmp = 1;
        for i in (1..res.len()).rev() {
            res[i] = res[i - 1] * tmp;
            tmp *= nums[i];
        }
        res[0] = tmp;
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![24, 12, 8, 6];
    assert_eq!(Solution::product_except_self(nums), res);
}