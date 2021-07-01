struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut cur = 0;
        for num in nums {
            cur += num;
            res = res.max(cur);
            if cur < 0 {
                cur = 0;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);
    let nums = vec![1];
    assert_eq!(Solution::max_sub_array(nums), 1);
}