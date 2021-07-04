struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];
        let mut res = 0_i32;
        for i in 0..n {
            if matrix[i][0] == '1' {
                dp[i][0] = 1;
                res = 1;
            }
        }
        for j in 0..m {
            if matrix[0][j] == '1' {
                dp[0][j] = 1;
                res = 1;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == '1' {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }
        res.pow(2)
    }
}

#[test]
fn test() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(Solution::maximal_square(matrix), 4);
}
