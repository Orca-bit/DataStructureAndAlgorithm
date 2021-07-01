struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let n = nums.len();
        if target < nums[0] || target > nums[n - 1] {
            return vec![-1, -1];
        }
        let search = |equal: bool| -> usize {
            let mut res = n;
            let mut l = 0;
            let mut r = n as i32 - 1;
            while l <= r {
                let mid = l + ((r - l) >> 1);
                if nums[mid as usize] > target || (equal && nums[mid as usize]>= target) {
                    r = mid - 1;
                    res = mid as usize;
                } else {
                    l = mid + 1;
                }
            }
            res
        };
        let left_index = search(true) as i32;
        let right_index = search(false) as i32 - 1;
        if left_index <= right_index
            && left_index >= 0
            && right_index < n as i32
            && nums[left_index as usize] == target
            && nums[right_index as usize] == target
        {
            return vec![left_index, right_index];
        }
        vec![-1, -1]
    }
}

#[test]
fn test() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let res = vec![3, 4];
    assert_eq!(Solution::search_range(nums, target), res);
    assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
}