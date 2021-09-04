use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut mask = 0;
        for i in (0..31).rev() {
            mask |= 1 << i;
            let mut hs = HashSet::new();
            for &x in &nums {
                hs.insert(x & mask);
            }
            let tmp = res | (1 << i);
            for &x in &hs {
                if hs.contains(&(x ^ tmp)) {
                    res = tmp;
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, 10, 5, 25, 2, 8];
    let res = 28;
    assert_eq!(Solution::find_maximum_xor(nums), res);
}
