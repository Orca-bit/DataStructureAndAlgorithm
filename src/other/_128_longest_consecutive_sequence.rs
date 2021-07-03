use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hs = nums.into_iter().collect::<HashSet<_>>();
        let mut res = 0;
        for &num in hs.iter() {
            if !hs.contains(&(num - 1)) {
                let mut cur = num;
                let mut len = 1;
                while hs.contains(&(cur + 1)) {
                    cur += 1;
                    len += 1;
                }
                res = res.max(len);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let res = 4;
    assert_eq!(Solution::longest_consecutive(nums), res);
    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let res = 9;
    assert_eq!(Solution::longest_consecutive(nums), res);
    let nums = vec![1, 2, 0, 1];
    let res = 3;
    assert_eq!(Solution::longest_consecutive(nums), res);
}
