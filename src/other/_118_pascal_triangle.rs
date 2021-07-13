struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut res = vec![vec![1]; num_rows];
        for i in 1..num_rows {
            for j in 1..i {
                let value = res[i - 1][j - 1] + res[i - 1][j];
                res[i].push(value);
            }
            res[i].push(1);
        }
        res
    }
}

#[test]
fn test() {
    let triangle_5 = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
    ];
    assert_eq!(Solution::generate(5), triangle_5);
}
