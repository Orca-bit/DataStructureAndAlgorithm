struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 2..n + 1 {
            for j in 1..i {
                dp[i] = dp[i].max(((i - j) * j) as i32).max((i - j) as i32 * dp[j]);
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 1;
    assert_eq!(Solution::integer_break(n), res);
    let n = 10;
    let res = 36;
    assert_eq!(Solution::integer_break(n), res);
}
