struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums[0..k as usize].iter().sum::<i32>();
        let mut res = sum;
        for i in k as usize..nums.len() {
            sum += nums[i];
            sum -= nums[i - k as usize];
            res = res.max(sum);
        }
        res as f64 / k as f64
    }
}

#[test]
fn test() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    assert_eq!(Solution::find_max_average(nums, k), 12.75);
}
