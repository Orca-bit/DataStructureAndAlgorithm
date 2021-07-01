struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = grid[0].clone();
        for i in 1..dp.len() {
            dp[i] += dp[i - 1];
        }
        for i in 1..n {
            dp[0] += grid[i][0];
            for j in 1..m {
                dp[j] = grid[i][j] + dp[j].min(dp[j - 1]);
            }
        }
        *dp.last().unwrap()
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let res = 7;
    assert_eq!(Solution::min_path_sum(grid), res);
}
