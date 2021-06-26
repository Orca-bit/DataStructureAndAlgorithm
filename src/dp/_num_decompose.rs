fn get_decompose_methods(num: i32) -> i32 {
    process(1, num)
}

fn process(pre: i32, rest: i32) -> i32 {
    if rest == 0 {
        return 1;
    }
    if rest < pre {
        return 0;
    }
    let mut res = 0;
    for i in pre..=rest {
        res += process(i, rest - i);
    }
    res
}

fn dp(num: i32) -> i32 {
    let n = num as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        dp[i][0] = 1;
    }
    dp[n][n] = 1;
    for i in (1..n).rev() {
        for j in i..=n {
            //for k in i..=j {
            //    dp[i][j] += dp[k][j - k];
            //}
            dp[i][j] = dp[i][j - i] + dp[i + 1][j];
        }
    }
    dp[1][n]
}

#[test]
fn test() {
    assert_eq!(get_decompose_methods(4), 5);
    assert_eq!(get_decompose_methods(7), dp(7));
}