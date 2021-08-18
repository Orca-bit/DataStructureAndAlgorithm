struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut mins = vec![i32::MAX; n];
        let mut maxs = vec![i32::MIN; m];
        for i in 0..n {
            for j in 0..m {
                mins[i] = mins[i].min(matrix[i][j]);
                maxs[j] = maxs[j].max(matrix[i][j]);
            }
        }
        let mut res = vec![];
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == mins[i] && matrix[i][j] == maxs[j] {
                    res.push(matrix[i][j]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32_1![[3, 7, 8], [9, 11, 13], [15, 16, 17]];
    let res = vec![15];
    assert_eq!(Solution::lucky_numbers(matrix), res);
    let matrix = vec_vec_i32_1![[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]];
    let res = vec![12];
    assert_eq!(Solution::lucky_numbers(matrix), res);
    let matrix = vec_vec_i32_1![[7, 8], [1, 2]];
    let res = vec![7];
    assert_eq!(Solution::lucky_numbers(matrix), res);
}
