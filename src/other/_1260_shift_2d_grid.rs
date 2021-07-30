struct Solution;

impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut v = vec![];
        for i in 0..n {
            for j in 0..m {
                v.push(grid[i][j]);
            }
        }
        let size = v.len();
        let mut k = size - (k as usize) % size;
        for i in 0..n {
            for j in 0..m {
                grid[i][j] = v[k % size];
                k += 1;
            }
        }
        grid
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32_1![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let k = 1;
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[9, 1, 2], [3, 4, 5], [6, 7, 8]];
    assert_eq!(Solution::shift_grid(grid, k), res);
    let grid = vec_vec_i32_1![[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]];
    let k = 4;
    let res: Vec<Vec<i32>> =
        vec_vec_i32_1![[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]];
    assert_eq!(Solution::shift_grid(grid, k), res);
}
