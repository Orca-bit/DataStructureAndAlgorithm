// n: 总位置数，m: 初始位置，k: 所需步数，p: 目标位置
// 方法一 暴力递归
fn walk1(n: i32, m: i32, k: i32, p: i32) -> i32 {
    if n < 2 || k < 1 || m < 1 || m > n || p < 1 || p > n {
        return 0;
    }
    process1(n, p, m, k)
}

fn process1(n: i32, p: i32, cur: i32, rest: i32) -> i32 {
    if rest == 0 {
        return if cur == p { 1 } else { 0 };
    }
    if cur == 1 {
        return process1(n, p, cur + 1, rest - 1);
    }
    if cur == n {
        return process1(n, p, cur - 1, rest - 1);
    }
    process1(n, p, cur - 1, rest - 1) + process1(n, p, cur + 1, rest - 1)
}

// 方法二 记忆化搜索
fn walk2(n: i32, m: i32, k: i32, p: i32) -> i32 {
    if n < 2 || k < 1 || m < 1 || m > n || p < 1 || p > n {
        return 0;
    }
    let mut dp = vec![vec![-1; k as usize + 1]; n as usize + 1];
    process2(n, p, m, k, &mut dp)
}

fn process2(n: i32, p: i32, cur: i32, rest: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[cur as usize][rest as usize] != -1 {
        return dp[cur as usize][rest as usize];
    }
    if rest == 0 {
        dp[cur as usize][rest as usize] = if cur == p { 1 } else { 0 };
    } else {
        if cur == 1 {
            dp[cur as usize][rest as usize] = process2(n, p, cur + 1, rest - 1, dp);
        } else if cur == n {
            dp[cur as usize][rest as usize] = process2(n, p, cur - 1, rest - 1, dp);
        } else {
            dp[cur as usize][rest as usize] =
                process2(n, p, cur - 1, rest - 1, dp) + process2(n, p, cur + 1, rest - 1, dp);
        }
    }
    dp[cur as usize][rest as usize]
}

// n: 总位置数，m: 初始位置，k: 所需步数，p: 目标位置
// 方法三 dp
fn walk_dp(n: i32, m: i32, k: i32, p: i32) -> i32 {
    if n < 2 || k < 1 || m < 1 || m > n || p < 1 || p > n {
        return 0;
    }
    let n = n as usize;
    let m = m as usize;
    let k = k as usize;
    let p = p as usize;
    let mut dp = vec![vec![0; k + 1]; n + 1]; // [cur][rest]
    dp[p][0] = 1;
    for j in 1..=k {
        for i in 1..=n {
            if i == 1 {
                dp[i][j] = dp[i + 1][j - 1];
            } else if i == n {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j - 1] + dp[i + 1][j - 1];
            }
        }
    }
    dp[m][k]
}

fn walk_dp_1d(n: i32, m: i32, k: i32, p: i32) -> i32 {
    if n < 2 || k < 1 || m < 1 || m > n || p < 1 || p > n {
        return 0;
    }
    let n = n as usize;
    let m = m as usize;
    let k = k as usize;
    let p = p as usize;
    let mut dp = vec![0; n + 1];
    dp[p] = 1;
    for _ in 1..=k {
        let mut old_pre = dp[1];
        for i in 1..=n {
            let old = dp[i];
            if i == 1 {
                dp[i] = dp[i + 1];
            } else if i == n {
                dp[i] = old_pre;
            } else {
                dp[i] = old_pre + dp[i + 1];
            }
            old_pre = old;
        }
    }
    dp[m]
}

#[test]
fn test() {
    assert_eq!(walk1(10, 4, 6, 8), walk2(10, 4, 6, 8));
    assert_eq!(walk1(10, 4, 6, 8), walk_dp(10, 4, 6, 8));
    assert_eq!(walk1(10, 4, 6, 8), walk_dp_1d(10, 4, 6, 8));
}
