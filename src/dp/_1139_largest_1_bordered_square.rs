struct Solution;

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut top = vec![vec![0; m]; n];
        let mut left = vec![vec![0; m]; n];
        let mut down = vec![vec![0; m]; n];
        let mut right = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    top[i][j] = 1 + if i > 0 { top[i - 1][j] } else { 0 };
                    left[i][j] = 1 + if j > 0 { left[i][j - 1] } else { 0 };
                }
            }
        }
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if grid[i][j] == 1 {
                    down[i][j] = 1 + if i < n - 1 { down[i + 1][j] } else { 0 };
                    right[i][j] = 1 + if j < m - 1 { right[i][j + 1] } else { 0 };
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                for k in 0..(i + 1).min(j + 1) {
                    if top[i][j] >= k + 1
                        && left[i][j] >= k + 1
                        && down[i - k][j - k] >= k + 1
                        && right[i - k][j - k] >= k + 1
                    {
                        res = res.max(k as i32 + 1);
                    }
                }
            }
        }
        res * res
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let res = 9;
    assert_eq!(Solution::largest1_bordered_square(grid), res);
    let grid = vec![vec![1, 1, 0, 0]];
    let res = 1;
    assert_eq!(Solution::largest1_bordered_square(grid), res);
}
