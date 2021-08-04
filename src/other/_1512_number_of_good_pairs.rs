use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut res = 0;
        for x in nums {
            let v: &mut i32 = map.entry(x).or_default();
            res += *v;
            *v += 1;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let res = 4;
    assert_eq!(Solution::num_identical_pairs(nums), res);
    let nums = vec![1, 1, 1, 1];
    let res = 6;
    assert_eq!(Solution::num_identical_pairs(nums), res);
    let nums = vec![1, 2, 3];
    let res = 0;
    assert_eq!(Solution::num_identical_pairs(nums), res);
}
