struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut res = nums[0];
        let mut min = nums[0];
        let mut max = nums[0];
        for num in nums.into_iter().skip(1) {
            let last_min = min;
            let last_max = max;
            min = (last_min * num).min(last_max * num).min(num);
            max = (last_max * num).max(last_min * num).max(num);
            res = res.max(max);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, -2, 4];
    assert_eq!(Solution::max_product(nums), 6);
    let nums = vec![-2, 0, -1];
    assert_eq!(Solution::max_product(nums), 0);
}