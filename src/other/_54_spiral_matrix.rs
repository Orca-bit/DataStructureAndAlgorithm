struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let mut row_lr = matrix.len() as i32 - 1;
        let mut col_lr = matrix[0].len() as i32 - 1;
        let mut row_ul = 0;
        let mut col_ul = 0;
        while row_ul <= row_lr && col_ul <= col_lr {
            Self::process(
                &mut res,
                row_ul as usize,
                col_ul as usize,
                row_lr as usize,
                col_lr as usize,
                &matrix,
            );
            row_ul += 1;
            row_lr -= 1;
            col_ul += 1;
            col_lr -= 1;
        }
        res
    }

    fn process(
        res: &mut Vec<i32>,
        row_ul: usize,
        col_ul: usize,
        row_lr: usize,
        col_lr: usize,
        matrix: &Vec<Vec<i32>>,
    ) {
        if row_lr == row_ul {
            for i in col_ul..=col_lr {
                res.push(matrix[row_ul][i]);
            }
        } else if col_lr == col_ul {
            for i in row_ul..=row_lr {
                res.push(matrix[i][col_ul]);
            }
        } else {
            let mut row_cur = row_ul;
            let mut col_cur = col_ul;
            while col_cur != col_lr {
                res.push(matrix[row_ul][col_cur]);
                col_cur += 1;
            }
            while row_cur != row_lr {
                res.push(matrix[row_cur][col_lr]);
                row_cur += 1;
            }
            while col_cur != col_ul {
                res.push(matrix[row_lr][col_cur]);
                col_cur -= 1;
            }
            while row_cur != row_ul {
                res.push(matrix[row_cur][col_ul]);
                row_cur -= 1;
            }
        }
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32_1![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(Solution::spiral_order(matrix), res);
    let matrix: Vec<Vec<i32>> = vec_vec_i32_1![[3], [2]];
    let res = vec![3, 2];
    assert_eq!(Solution::spiral_order(matrix), res);
}
