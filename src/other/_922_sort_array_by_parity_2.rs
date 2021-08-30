struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut i = 0;
        let mut j = 1;
        while i < n && j < n {
            while i < n && nums[i] & 1 == 0 {
                i += 2;
            }
            while j < n && nums[j] & 1 == 1 {
                j += 2;
            }
            if i < n && j < n {
                nums.swap(i, j);
            }
        }
        nums
    }
}

#[test]
fn test() {
    let a = vec![4, 2, 5, 7];
    let b = vec![4, 5, 2, 7];

    assert_eq!(Solution::sort_array_by_parity_ii(a), b);
}
