struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fast = nums[nums[0] as usize];
        let mut slow = nums[0];
        while slow != fast {
            fast = nums[nums[fast as usize] as usize];
            slow = nums[slow as usize];
        }
        fast = 0;
        while slow != fast {
            fast = nums[fast as usize];
            slow = nums[slow as usize];
        }
        slow
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 4, 2, 2];
    let res = 2;
    assert_eq!(Solution::find_duplicate(nums), res);
    let nums = vec![3, 1, 3, 4, 2];
    let res = 3;
    assert_eq!(Solution::find_duplicate(nums), res);
}