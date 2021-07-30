struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut sorted = vec!["".to_string(); n];
        let a: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        let m =a[0].len();
        for j in 0..m {
            for i in 0..n {
                sorted[i].push(a[i][j]);
            }
            if sorted.windows(2).any(|w| w[0] > w[1]) {
                for i in 0..n {
                    sorted[i].pop();
                }
            }
        }
        (m - sorted[0].len()) as i32
    }
}

#[test]
fn test() {
    let a = vec!["ca".to_string(), "bb".to_string(), "ac".to_string()];
    let res = 1;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec!["xc".to_string(), "yb".to_string(), "za".to_string()];
    let res = 0;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
    let res = 3;
    assert_eq!(Solution::min_deletion_size(a), res);
}
