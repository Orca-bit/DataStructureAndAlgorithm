struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut next = 0;
        let mut jump = 0;
        for i in 0..nums.len() {
            if cur < i as i32 {
                jump += 1;
                cur = next;
            }
            next = next.max(i as i32 + nums[i]);
        }
        jump
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 1, 4];
    let res = 2;
    assert_eq!(Solution::jump(nums), res);
}