struct NumMatrix {
    pre_sum: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        if rows == 0 {
            return Self {
                pre_sum: vec![],
                rows,
                cols: 0,
            };
        }
        let cols = matrix[0].len();
        let mut pre_sum = vec![vec![0; cols + 1]; rows + 1];
        for i in 0..rows {
            for j in 0..cols {
                pre_sum[i + 1][j + 1] = matrix[i][j];
                pre_sum[i + 1][j + 1] += pre_sum[i + 1][j];
                pre_sum[i + 1][j + 1] += pre_sum[i][j + 1];
                pre_sum[i + 1][j + 1] -= pre_sum[i][j];
            }
        }
        Self {
            pre_sum,
            rows,
            cols,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1) = (row1 as usize, col1 as usize);
        let (r2, c2) = (row2 as usize + 1, col2 as usize + 1);
        let mut res = self.pre_sum[r2][c2];
        res -= self.pre_sum[r2][c1];
        res -= self.pre_sum[r1][c2];
        res += self.pre_sum[r1][c1];
        res
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[test]
fn test() {
    let matrix = vec_vec_i32_1![
        [3, 0, 1, 4, 2],
        [5, 6, 3, 2, 1],
        [1, 2, 0, 1, 5],
        [4, 1, 0, 1, 7],
        [1, 0, 3, 0, 5]
    ];
    let nm = NumMatrix::new(matrix);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
}
