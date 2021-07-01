struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let mut dp = vec![1; m.min(n) as usize];
        let times = if dp.len() == m as usize { n } else { m };
        for _ in 1..times {
            for i in 1..dp.len() {
                dp[i] += dp[i - 1];
            }
        }
        *dp.last().unwrap()
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 2;
    let res = 3;
    assert_eq!(Solution::unique_paths(m, n), res);
    let m = 7;
    let n = 3;
    let res = 28;
    assert_eq!(Solution::unique_paths(m, n), res);
}
