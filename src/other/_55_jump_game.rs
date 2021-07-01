struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let mut max = nums[0];
        for j in 1..nums.len() {
            if j as i32 > max {
                return false;
            }
            max = max.max(j as i32 + nums[j]);
        }
        true
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 1, 4];
    let res = true;
    assert_eq!(Solution::can_jump(nums), res);
    let nums = vec![3, 2, 1, 0, 4];
    let res = false;
    assert_eq!(Solution::can_jump(nums), res);
}