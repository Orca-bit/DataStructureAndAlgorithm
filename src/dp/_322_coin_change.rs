use std::cmp::min;

struct Solution;

impl Solution {
    fn coin_change1(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.is_empty() || amount < 0 {
            return -1;
        }
        let mut dp = vec![vec![-2; amount as usize + 1]; coins.len() + 1];
        Self::process1(&coins, 0, amount, &mut dp)
    }

    fn process1(coins: &[i32], index: usize, rest: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if rest < 0 {
            return -1;
        }
        if dp[index][rest as usize] != -2 {
            return dp[index][rest as usize];
        }
        if index == coins.len() {
            dp[index][rest as usize] = if rest == 0 { 0 } else { -1 };
        } else {
            dp[index][rest as usize] = -1;
            let mut k = 0;
            while k * coins[index] <= rest {
                let next = Self::process1(coins, index + 1, rest - k * coins[index], dp);
                if next != -1 {
                    dp[index][rest as usize] = if dp[index][rest as usize] == -1 {
                        k + next
                    } else {
                        min(dp[index][rest as usize], k + next)
                    };
                }
                k += 1;
            }
        }
        dp[index][rest as usize]
    }

    fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.is_empty() || amount < 0 {
            return -1;
        }
        let mut dp = vec![vec![0; amount as usize + 1]; coins.len() + 1];
        for rest in 1..=amount as usize {
            dp[coins.len()][rest] = -1;
        }
        for index in (0..coins.len()).rev() {
            for rest in 0..=amount as usize {
                dp[index][rest] = -1;
                if dp[index + 1][rest] != -1 {
                    dp[index][rest] = dp[index + 1][rest];
                }
                if rest as i32 - coins[index] >= 0 && dp[index][rest - coins[index] as usize] != -1
                {
                    if dp[index][rest] == -1 {
                        dp[index][rest] = dp[index][rest - coins[index] as usize] + 1;
                    } else {
                        dp[index][rest] =
                            min(dp[index][rest], dp[index][rest - coins[index] as usize] + 1);
                    }
                }
            }
        }
        dp[0][amount as usize]
    }
}

#[test]
fn test() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    let res = 3;
    assert_eq!(Solution::coin_change_dp(coins, amount), res);
    let coins = vec![2];
    let amount = 3;
    let res = -1;
    assert_eq!(Solution::coin_change1(coins, amount), res);
}
