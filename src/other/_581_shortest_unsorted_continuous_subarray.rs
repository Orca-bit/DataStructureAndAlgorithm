struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let n = nums.len();
        let mut min = nums[n - 1];
        let mut no_min_index = -1;
        for i in (0..n - 1).rev() {
            if nums[i] > min {
                no_min_index = i as i32;
            } else {
                min = min.min(nums[i]);
            }
        }
        if no_min_index == -1 {
            return 0;
        }
        let mut no_max_index = -1;
        let mut max = nums[0];
        for i in 1..n {
            if nums[i] < max {
                no_max_index = i as i32;
            } else {
                max = max.max(nums[i]);
            }
        }
        no_max_index - no_min_index + 1
    }
}

#[test]
fn test() {
    let nums = vec![2, 6, 4, 8, 10, 9, 15];
    assert_eq!(Solution::find_unsorted_subarray(nums), 5);
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::find_unsorted_subarray(nums), 0);
}
