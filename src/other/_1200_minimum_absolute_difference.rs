struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let min = arr.windows(2).fold(i32::MAX, |x, v| x.min(v[1] - v[0]));
        let mut res = vec![];
        for v in arr.windows(2) {
            if v[1] - v[0] == min {
                res.push(vec![v[0], v[1]]);
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![4, 2, 1, 3];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[1, 2], [2, 3], [3, 4]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
    let arr = vec![1, 3, 6, 10, 15];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[1, 3]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
    let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[-14, -10], [19, 23], [23, 27]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
}
