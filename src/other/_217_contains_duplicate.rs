use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let hs = nums.iter().copied().collect::<HashSet<_>>();
        !(hs.len() == nums.len())
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_duplicate(nums), true);
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::contains_duplicate(nums), false);
    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert_eq!(Solution::contains_duplicate(nums), true);
}