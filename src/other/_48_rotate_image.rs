struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        if n < 2 {
            return;
        }
        let mut top_row = 0;
        let mut top_col = 0;
        let mut floor_row = n - 1;
        let mut floor_col = n - 1;
        while top_row < floor_row {
            Self::process(matrix, top_row, top_col, floor_row, floor_col);
            top_row += 1;
            top_col += 1;
            floor_row -= 1;
            floor_col -= 1;
        }
    }

    fn process(
        matrix: &mut Vec<Vec<i32>>,
        top_row: usize,
        top_col: usize,
        floor_row: usize,
        floor_col: usize,
    ) {
        let mut n = floor_row - top_row;
        for i in 0..n {
            let tmp = matrix[top_row][top_col + i];
            matrix[top_row][top_col + i] = matrix[floor_row - i][top_col];
            matrix[floor_row - i][top_col] = matrix[floor_row][floor_col - i];
            matrix[floor_row][floor_col - i] = matrix[top_row + i][floor_col];
            matrix[top_row + i][floor_col] = tmp;
        }
    }
}

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32_1![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[7, 4, 1], [8, 5, 2], [9, 6, 3]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, res);
}