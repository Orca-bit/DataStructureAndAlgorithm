struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut v = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                v[i] = v[i].max(v[i - 1] + 1);
            }
        }
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                v[i] = v[i].max(v[i + 1] + 1);
            }
        }
        v.iter().sum::<i32>()
    }
}

#[test]
fn test() {
    let ratings = vec![1, 0, 2];
    let res = 5;
    assert_eq!(Solution::candy(ratings), res);
    let ratings = vec![1, 2, 2];
    let res = 4;
    assert_eq!(Solution::candy(ratings), res);
    let ratings = vec![1, 3, 2, 2, 1];
    let res = 7;
    assert_eq!(Solution::candy(ratings), res);
}
