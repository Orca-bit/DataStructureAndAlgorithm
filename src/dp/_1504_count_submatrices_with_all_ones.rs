struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut dp = vec![vec![0; m]; n];
        let mut res = 0;
        for i in 0..n {
            dp[i][0] = mat[i][0];
        }
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 0 {
                    continue;
                }
                if j > 0 {
                    dp[i][j] = dp[i][j - 1] + 1;
                }
                let mut min = i32::MAX;
                for k in (0..=i).rev() {
                    if dp[k][j] == 0 {
                        break;
                    }
                    min = min.min(dp[k][j]);
                    res += min;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
    let res = 13;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];
    let res = 24;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec![vec![1, 1, 1, 1, 1, 1]];
    let res = 21;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]];
    let res = 5;
    assert_eq!(Solution::num_submat(mat), res);
}
