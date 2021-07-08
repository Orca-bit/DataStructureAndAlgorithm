struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            if nums[l] == l as i32 + 1 {
                l += 1;
            } else if nums[l] <= l as i32
                || nums[l] > r as i32
                || nums[l] == nums[nums[l] as usize - 1]
            {
                nums[l] = nums[r - 1];
                r -= 1;
            } else {
                let index = nums[l] as usize - 1;
                nums.swap(l, index);
            }
        }
        l as i32 + 1
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 0];
    let res = 3;
    assert_eq!(Solution::first_missing_positive(nums), res);
    let nums = vec![3, 4, -1, 1];
    let res = 2;
    assert_eq!(Solution::first_missing_positive(nums), res);
    let nums = vec![7, 8, 9, 11, 12];
    let res = 1;
    assert_eq!(Solution::first_missing_positive(nums), res);
}
