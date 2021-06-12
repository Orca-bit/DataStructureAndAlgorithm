use std::cmp::{max, min};

struct Solution;

impl Solution {
    fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring: Vec<_> = ring.chars().collect();
        let key: Vec<_> = key.chars().collect();
        let mut index_table = vec![vec![]; 26];
        for (i, &c) in ring.iter().enumerate() {
            let index = (c as u8 - b'a') as usize;
            index_table[index].push(i);
        }
        let mut dp = vec![vec![-1; key.len() + 1]; ring.len() + 1];
        Self::process(&mut dp, 0, 0, &index_table, &ring, &key) + key.len() as i32
    }

    fn process(
        dp: &mut Vec<Vec<i32>>,
        ring_index: usize,
        key_index: usize,
        index_table: &Vec<Vec<usize>>,
        ring: &[char],
        key: &[char],
    ) -> i32 {
        if dp[ring_index][key_index] != -1 {
            return dp[ring_index][key_index];
        }
        if key_index == key.len() {
            dp[ring_index][key_index] = 0;
        } else {
            let target = key[key_index];
            let target_index = (target as u8 - b'a') as usize;
            dp[ring_index][key_index] = i32::MAX;
            for &i in index_table[target_index].iter() {
                let step1 = (i as i32 - ring_index as i32).abs();
                let step2 = ring.len() as i32 - step1;
                let min_step = min(step1, step2);
                dp[ring_index][key_index] = min(
                    dp[ring_index][key_index],
                    min_step + Self::process(dp, i, key_index + 1, index_table, ring, key),
                );
            }
        }
        dp[ring_index][key_index]
    }

    fn dp(ring: String, key: String) -> i32 {
        let ring: Vec<_> = ring.chars().collect();
        let key: Vec<_> = key.chars().collect();
        let n = ring.len();
        let m = key.len();
        let mut index_table = vec![vec![]; 26];
        for (i, &c) in ring.iter().enumerate() {
            let index = (c as u8 - b'a') as usize;
            index_table[index].push(i);
        }
        let mut dp = vec![vec![0; m + 1]; n];
        for j in (0..m).rev() {
            for i in 0..n {
                let target = key[j];
                let target_index = (target as u8 - b'a') as usize;
                dp[i][j] = i32::MAX;
                for &k in index_table[target_index].iter() {
                    let step1 = (k as i32 - i as i32).abs();
                    let step2 = n as i32 - step1;
                    let min_step = min(step1, step2);
                    dp[i][j] = min(dp[i][j], min_step + dp[k][j + 1]);
                }
            }
        }
        dp[0][0] + m as i32
    }
}

#[test]
fn test() {
    let ring = "godding".to_string();
    let key = "gd".to_string();
    let res = 4;
    assert_eq!(Solution::find_rotate_steps(ring, key), res);
    let ring = "godding".to_string();
    let key = "gd".to_string();
    let res = 4;
    assert_eq!(Solution::dp(ring, key), res);
}
