struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut descent = true;
        let mut ascent = true;
        let n = nums.len();
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                ascent = false;
            }
            if nums[i] < nums[i + 1] {
                descent = false;
            }
        }
        descent || ascent
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 2, 3];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![6, 5, 4, 4];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![1, 3, 2];
    assert_eq!(Solution::is_monotonic(a), false);
    let a = vec![1, 2, 4, 5];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![1, 1, 1];
    assert_eq!(Solution::is_monotonic(a), true);
}
