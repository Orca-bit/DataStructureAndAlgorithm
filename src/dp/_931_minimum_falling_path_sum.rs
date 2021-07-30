struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = matrix[0].clone();
        for i in 1..n {
            let mut old_i_1 = dp[0];
            dp[0] = dp[0].min(dp[1]) + matrix[i][0];
            let mut old_i = dp[1];
            for j in 1..m - 1 {
                dp[j] = dp[j + 1].min(old_i).min(old_i_1) + matrix[i][j];
                old_i_1 = old_i;
                old_i = dp[j + 1];
            }
            dp[m - 1] = old_i.min(old_i_1) + matrix[i][m - 1];
        }
        *dp.iter().min().unwrap()
    }
}

#[test]
fn test() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = 12;
    assert_eq!(Solution::min_falling_path_sum(a), res);
}
