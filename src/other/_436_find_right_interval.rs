use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut res = vec![-1; n];
        let mut btm = BTreeMap::new();
        for i in 0..n {
            let l = intervals[i][0];
            btm.insert(l, i);
        }
        for i in 0..n {
            let r = intervals[i][1];
            for (_, &j) in btm.range(r..).take(1) {
                res[i] = j as i32;
            }
        }
        res
    }
}

#[test]
fn test() {
    let intervals = vec_vec_i32_1![[1, 2]];
    let res = vec![-1];
    assert_eq!(Solution::find_right_interval(intervals), res);
    let intervals = vec_vec_i32_1![[3, 4], [2, 3], [1, 2]];
    let res = vec![-1, 0, 1];
    assert_eq!(Solution::find_right_interval(intervals), res);
    let intervals = vec_vec_i32_1![[1, 4], [2, 3], [3, 4]];
    let res = vec![-1, 2, -1];
    assert_eq!(Solution::find_right_interval(intervals), res);
}
