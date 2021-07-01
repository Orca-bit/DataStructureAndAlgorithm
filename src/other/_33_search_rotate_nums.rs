struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[l as usize] <= nums[mid as usize] {
                if nums[l as usize] <= target && target <= nums[mid as usize] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                if nums[mid as usize] <= target && target <= nums[r as usize] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(Solution::search(nums, target), 4);
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    assert_eq!(Solution::search(nums, target), -1);
    let nums = vec![3, 5, 1];
    let target = 3;
    assert_eq!(Solution::search(nums, target), 0);
    let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
    let target = 8;
    assert_eq!(Solution::search(nums, target), 4);
}
