struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < nums[mid + 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1];
    let res = 2;
    assert_eq!(Solution::find_peak_element(nums), res);
    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    let res = 5;
    assert_eq!(Solution::find_peak_element(nums), res);
}