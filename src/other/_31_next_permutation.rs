struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.is_empty() || nums.len() == 1 {
            return;
        }
        let mut index1 = None;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                index1 = Some(i);
                break;
            }
        }
        if index1.is_none() {
            nums.reverse();
            return;
        }
        let index1 = index1.unwrap();
        let mut index2 = nums.len() - 1;
        for i in (index1 + 1..nums.len()).rev() {
            if nums[i] > nums[index1] {
                index2 = i;
                break;
            }
        }
        nums.swap(index1, index2);
        nums[index1 + 1..].reverse();
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 2, 3];
    let res = vec![1, 3, 2];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![3, 2, 1];
    let res = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![1, 1, 5];
    let res = vec![1, 5, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
}