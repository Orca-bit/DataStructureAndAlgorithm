struct Solution;

impl Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut left = 0;
        let mut right = n * m;
        while right > left {
            let mid = left + (right - left) / 2;
            if matrix[mid / m][mid % m] == target {
                return true;
            }
            if matrix[mid / m][mid % m] > target {
                right =  mid;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}

use super::util::*;

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32_1![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]];
    let target = 3;
    let res = true;
    assert_eq!(Solution::search_matrix(matrix, target), res);
}

