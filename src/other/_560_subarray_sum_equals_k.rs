use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_sum_map = HashMap::new();
        pre_sum_map.insert(0, 1);
        let mut res = 0;
        let mut sum = 0;
        for &num in nums.iter() {
            sum += num;
            if let Some(&ans) = pre_sum_map.get(&(sum - k)) {
                res += ans;
            }
            *pre_sum_map.entry(sum).or_default() += 1;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let k = 2;
    assert_eq!(Solution::subarray_sum(nums, k), 2);
    let nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let k = 0;
    assert_eq!(Solution::subarray_sum(nums, k), 55);
}
