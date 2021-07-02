struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let mut left = -1;
        let mut right = nums.len();
        let mut index = 0;
        while index < right {
            if nums[index] == 0 {
                left += 1;
                nums.swap(index, left as usize);
                index += 1;
            } else if nums[index] == 2 {
                right -= 1;
                nums.swap(index, right);
            } else {
                index += 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    let res = vec![0, 0, 1, 1, 2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, res);
}