struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);
        let dp_size = k + max_pts;
        let mut dp = vec![0.; dp_size];
        for i in k..dp_size.min(n + 1) {
            dp[i] = 1.;
        }
        let mut sum = dp[k..].iter().sum::<f64>();
        for i in (0..k).rev() {
            dp[i] = sum / max_pts as f64;
            sum += dp[i];
            sum -= dp[i + max_pts];
        }
        dp[0]
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let n = 10;
    let k = 1;
    let w = 10;
    let res = 1.0;
    assert_eq!(Solution::new21_game(n, k, w), res);
    let n = 6;
    let k = 1;
    let w = 10;
    let res = 0.6;
    assert_eq!(Solution::new21_game(n, k, w), res);
    let n = 21;
    let k = 17;
    let w = 10;
    let res = 0.732777;
    assert_approx_eq!(Solution::new21_game(n, k, w), res);
}
