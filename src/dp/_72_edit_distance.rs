use std::mem::swap;

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_cost(word1, word2, 1, 1, 1)
    }

    fn min_cost(word1: String, word2: String, mut ic: i32, mut dc: i32, rc: i32) -> i32 {
        let mut word1 = word1.chars().collect::<Vec<_>>();
        let mut word2 = word2.chars().collect::<Vec<_>>();
        if word1.len() < word2.len() {
            swap(&mut word1, &mut word2);
            swap(&mut ic, &mut dc);
        }
        // let rows = word1.len() + 1;
        // let cols = word2.len() + 1;
        // let mut dp = vec![vec![i32::MAX; cols]; rows];
        let mut dp = vec![0; word2.len() + 1];
        // for i in 0..rows {
        //     dp[i][0] = dc * i as i32;
        // }
        // for j in 1..cols {
        //     dp[0][j] = ic * j as i32;
        // }
        for j in 1..dp.len() {
            dp[j] = ic * j as i32;
        }
        // for i in 1..rows {
        //     for j in 1..cols {
        //         if word1[i - 1] == word2[j - 1] {
        //             dp[i][j] = dp[i - 1][j - 1];
        //         } else {
        //             dp[i][j] = dp[i - 1][j - 1] + rc;
        //         }
        //         dp[i][j] = dp[i][j].min((dp[i - 1][j] + dc).min(dp[i][j - 1] + ic));
        //     }
        // }
        for i in 1..word1.len() + 1 {
            let mut pre = dp[0];
            dp[0] = dc * i as i32;
            for j in 1..dp.len() {
                let tmp = dp[j];
                if word1[i - 1] == word2[j - 1] {
                    dp[j] = pre;
                } else {
                    dp[j] = pre + rc;
                }
                dp[j] = dp[j].min(tmp + dc).min(dp[j - 1] + ic);
                pre = tmp;
            }
        }
        *dp.last().unwrap()
    }
}

#[test]
fn test() {
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    let res = 3;
    assert_eq!(Solution::min_distance(word1, word2), res);
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    let res = 5;
    assert_eq!(Solution::min_distance(word1, word2), res);
}