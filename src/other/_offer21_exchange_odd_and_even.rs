struct Solution;

impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] & 1 == 1 {
                left += 1;
                continue;
            }
            if nums[right] & 1 == 0 {
                right -= 1;
                continue;
            }
            nums.swap(left, right);
        }
        nums
    }
}

#[test]
fn test() {
    assert_eq!(Solution::exchange(vec![1, 2, 3, 4]), vec![1, 3, 2, 4]);
}
