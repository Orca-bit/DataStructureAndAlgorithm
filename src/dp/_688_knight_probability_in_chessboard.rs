struct Solution;

impl Solution {
    fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        Self::process(n, k, r, c)
    }
    // 该方法直接计算概率，
    // 如果计算步数再计算概率的话，测试用例会溢出
    fn process(n: i32, step: i32, r: i32, c: i32) -> f64 {
        if r < 0 || r >= n || c < 0 || c >= n {
            return 0.;
        }
        if step == 0 {
            return 1.;
        }
        0.125 * Self::process(n, step - 1, r + 1, c + 2)
            + 0.125 * Self::process(n, step - 1, r + 1, c - 2)
            + 0.125 * Self::process(n, step - 1, r + 2, c + 1)
            + 0.125 * Self::process(n, step - 1, r + 2, c - 1)
            + 0.125 * Self::process(n, step - 1, r - 1, c + 2)
            + 0.125 * Self::process(n, step - 1, r - 1, c - 2)
            + 0.125 * Self::process(n, step - 1, r - 2, c + 1)
            + 0.125 * Self::process(n, step - 1, r - 2, c - 1)
    }

    // 动态规划版本
    fn dp(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0.; n]; n]; k as usize + 1];
        for i in 0..n {
            for j in 0..n {
                dp[0][i][j] = 1.;
            }
        }
        for step in 1..=k as usize {
            for i in 0..n {
                for j in 0..n {
                    let mut value = dp[step][i][j];
                    {
                        let n = n as i32;
                        let i = i as i32;
                        let j = j as i32;
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i + 1, j + 2);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i + 1, j - 2);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i + 2, j + 1);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i + 2, j - 1);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i - 1, j + 2);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i - 1, j - 2);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i - 2, j + 1);
                        value += 0.125 * Self::get_value(&dp, n, step - 1, i - 2, j - 1);
                    }
                    dp[step][i][j] = value;
                }
            }
        }
        dp[k as usize][row as usize][column as usize]
    }

    fn get_value(dp: &Vec<Vec<Vec<f64>>>, n: i32, step: usize, i: i32, j: i32) -> f64 {
        if i < 0 || i >= n || j < 0 || j >= n {
            return 0.;
        }
        dp[step][i as usize][j as usize]
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 2;
    let r = 0;
    let c = 0;
    let res = 0.0625;
    assert_eq!(Solution::dp(n, k, r, c), res);
    let n = 3;
    let k = 1;
    let r = 1;
    let c = 1;
    let res = 0.0;
    assert_eq!(Solution::knight_probability(n, k, r, c), res);
}
