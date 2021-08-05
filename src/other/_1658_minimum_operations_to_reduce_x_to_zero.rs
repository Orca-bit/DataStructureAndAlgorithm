use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let sum = nums.iter().sum::<i32>();
        let target = sum - x;
        if target == 0 {
            return n as i32;
        }
        let mut map = HashMap::new();
        map.insert(0, 0);
        let mut prev = 0;
        let mut max = 0;
        for i in 0..n {
            prev += nums[i];
            map.insert(prev, i + 1);
            if let Some(&j) = map.get(&(prev - target)) {
                max = max.max(i - j + 1);
            }
        }
        if max == 0 {
            -1
        } else {
            (n - max) as i32
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 4, 2, 3];
    let x = 5;
    let res = 2;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![5, 6, 7, 8, 9];
    let x = 4;
    let res = -1;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![3, 2, 20, 1, 1, 3];
    let x = 10;
    let res = 5;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![
        8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904, 8819, 1231,
        6309,
    ];
    let x = 134365;
    let res = 16;
    assert_eq!(Solution::min_operations(nums, x), res);
}
