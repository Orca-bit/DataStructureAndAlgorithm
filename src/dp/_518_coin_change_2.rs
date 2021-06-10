struct Solution;

impl Solution {
    fn change(amount: i32, coins: Vec<i32>) -> i32 {
       Self::process(&coins, 0, amount)
    }

    fn process(coins: &[i32], index: usize, rest: i32) -> i32 {
        if rest < 0 {
            return 0;
        }
        if index == coins.len() {
            return if rest == 0 {1} else { 0 };
        }
        let mut res = 0;
        let mut k = 0;
        while k * coins[index] <= rest {
            res += Self::process(coins, index + 1, rest - k * coins[index]);
            k += 1;
        }
        res
    }

    fn dp(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        let amount = amount as usize;
        let mut dp = vec![vec![0; amount + 1]; n + 1];
        dp[n][0] = 1;
        for i in (0..n).rev() {
            for j in 0..=amount {
                dp[i][j] = dp[i + 1][j];
                if j as i32 - coins[i] >= 0 {
                    dp[i][j] += dp[i][j - coins[i] as usize];
                }
            }
        }
        dp[0][amount]
    }
}

#[test]
fn test() {
    let amount = 5;
    let coins = vec![1, 2, 5];
    let res = 4;
    assert_eq!(Solution::dp(amount, coins), res);
    let amount = 3;
    let coins = vec![2];
    let res = 0;
    assert_eq!(Solution::change(amount, coins), res);
    let amount = 10;
    let coins = vec![10];
    let res = 1;
    assert_eq!(Solution::change(amount, coins), res);
}