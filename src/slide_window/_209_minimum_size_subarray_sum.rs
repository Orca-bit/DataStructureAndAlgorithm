struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut l = 0;
        let mut sum = 0;
        for r in 0..nums.len() {
            sum += nums[r];
            while sum >= target {
                res = res.min((r - l) as i32 + 1);
                sum -= nums[l];
                l += 1;
            }
        }
        if res == i32::MAX {
            0
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let s = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let res = 2;
    assert_eq!(Solution::min_sub_array_len(s, nums), res);
    let s = 4;
    let nums = vec![1, 4, 4];
    let res = 1;
    assert_eq!(Solution::min_sub_array_len(s, nums), res);
}
