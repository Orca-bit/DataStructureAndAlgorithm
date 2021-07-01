struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals;
        }
        let mut res = vec![];
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut s = intervals[0][0];
        let mut e = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] > e {
                res.push(vec![s, e]);
                s = interval[0];
                e = interval[1];
            } else {
                e = e.max(interval[1]);
            }
        }
        res.push(vec![s, e]);
        res
    }
}

#[test]
fn test() {
    let intervals: Vec<Vec<i32>> = vec_vec_i32_1![[1, 3], [2, 6], [8, 10], [15, 18]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[1, 6], [8, 10], [15, 18]];
    assert_eq!(Solution::merge(intervals), res);
    let intervals: Vec<Vec<i32>> = vec_vec_i32_1![[1, 4], [4, 5]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[1, 5]];
    assert_eq!(Solution::merge(intervals), res);
}