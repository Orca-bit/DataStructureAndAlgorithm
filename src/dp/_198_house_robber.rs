struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }
        let n = nums.len();
        let mut pre2 = nums[0];
        let mut pre1 = nums[1].max(nums[0]);
        for i in 2..n {
            let cur = pre1.max(pre2 + nums[i]);
            pre2 = pre1;
            pre1 = cur;
        }
        pre1
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 1, 2];
    assert_eq!(Solution::rob(nums), 4);
    let nums = vec![2, 7, 9, 3, 1];
    assert_eq!(Solution::rob(nums), 12);
}