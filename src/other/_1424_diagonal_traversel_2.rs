struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        for i in 0..n {
            let m = nums[i].len();
            for j in 0..m {
                let k = i + j;
                if res.len() == k {
                    res.push(vec![]);
                }
                res[k].push(nums[i][j]);
            }
        }
        res.into_iter().flat_map(|x| x.into_iter().rev()).collect()
    }
}

#[test]
fn test() {
    let nums = vec_vec_i32_1![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 4, 2, 7, 5, 3, 8, 6, 9];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32_1![
        [1, 2, 3, 4, 5],
        [6, 7],
        [8],
        [9, 10, 11],
        [12, 13, 14, 15, 16]
    ];
    let res = vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32_1![[1, 2, 3], [4], [5, 6, 7], [8], [9, 10, 11]];
    let res = vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32_1![[1, 2, 3, 4, 5, 6]];
    let res = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::find_diagonal_order(nums), res);
}
