struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut row0 = false;
        let mut col0 = false;
        for i in 0..m {
            if matrix[0][i] == 0 {
                row0 = true;
                break;
            }
        }
        for i in 0..n {
            if matrix[i][0] == 0 {
                col0 = true;
                break;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if row0 {
            for i in 0..m {
                matrix[0][i] = 0;
            }
        }
        if col0 {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32_1![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[1, 0, 1], [0, 0, 0], [1, 0, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, res);
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32_1![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, res);
}