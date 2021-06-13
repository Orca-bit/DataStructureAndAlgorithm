struct Solution;

impl Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut index = (0, m as i32 - 1);
        while index.0 < n as i32 && index.1 >= 0 {
            if matrix[index.0 as usize][index.1 as usize] > target {
                index.1 -= 1;
            } else if matrix[index.0 as usize][index.1 as usize] < target {
                index.0 += 1;
            } else {
                return true;
            }
        }
        false
    }
}

use super::util::*;
#[test]
fn test() {
    let matrix = vec_vec_i32_1![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30]
    ];
    let target = 5;
    let res = true;
    assert_eq!(Solution::search_matrix(matrix, target), res);
    let matrix = vec_vec_i32_1![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30]
    ];
    let target = 20;
    let res = false;
    assert_eq!(Solution::search_matrix(matrix, target), res);
}