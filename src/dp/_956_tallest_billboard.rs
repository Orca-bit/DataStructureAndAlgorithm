use std::collections::HashMap;
use std::mem::swap;

struct Solution;

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        Self::process(0, 0, 0, rods.as_slice(), &mut map)
    }

    fn process(
        index: usize,
        height1: i32,
        height2: i32,
        rods: &[i32],
        map: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if let Some(&res) = map.get(&(index, height1, height2)) {
            res
        } else {
            let res = if index == rods.len() {
                if height1 == height2 {
                    height1
                } else {
                    0
                }
            } else {
                let p1 = Self::process(index + 1, height1, height2, rods, map);
                let p2 = Self::process(index + 1, height1 + rods[index], height2, rods, map);
                let p3 = Self::process(index + 1, height1, height2 + rods[index], rods, map);
                p1.max(p2).max(p3)
            };
            map.insert((index, height1, height2), res);
            res
        }
    }

    fn dp(rods: Vec<i32>) -> i32 {
        let n = rods.len();
        let m = rods.iter().sum::<i32>() as usize;
        let mut dp1 = vec![vec![0; m + 1]; m + 1];
        let mut dp2 = vec![vec![0; m + 1]; m + 1];
        for i in 0..m + 1 {
            dp2[i][i] = i as i32;
        }
        for i in (0..n).rev() {
            for j in 0..m + 1 - rods[i] as usize {
                for k in 0..m + 1 - rods[i] as usize {
                    let p1 = dp2[j][k];
                    let p2 = dp2[j + rods[i] as usize][k];
                    let p3 = dp2[j][k + rods[i] as usize];
                    dp1[j][k] = p1.max(p2).max(p3);
                }
            }
            swap(&mut dp1, &mut dp2);
        }
        dp2[0][0]
    }

    fn dp2(rods: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        hm.insert(0, 0);
        for rod in rods {
            let tmp = hm.clone();
            for (sum, positive) in tmp {
                hm.insert(
                    sum + rod,
                    (positive + rod).max(*hm.get(&(sum + rod)).unwrap_or(&0)),
                );
                hm.insert(sum - rod, positive.max(*hm.get(&(sum - rod)).unwrap_or(&0)));
            }
        }
        hm[&0]
    }
}

#[test]
fn test() {
    let rods = vec![1, 2, 3, 6];
    assert_eq!(Solution::tallest_billboard(rods), 6);
    let rods = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::tallest_billboard(rods), 10);
    let rods = vec![1, 2];
    assert_eq!(Solution::tallest_billboard(rods), 0);
    let rods = vec![3, 4, 3, 3, 2];
    assert_eq!(Solution::tallest_billboard(rods), 6);
    let rods = vec![1, 2, 3, 6];
    assert_eq!(Solution::dp(rods), 6);
    let rods = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::dp(rods), 10);
    let rods = vec![1, 2];
    assert_eq!(Solution::dp(rods), 0);
    let rods = vec![3, 4, 3, 3, 2];
    assert_eq!(Solution::dp(rods), 6);
    let rods = vec![1, 2, 3, 6];
    assert_eq!(Solution::dp2(rods), 6);
    let rods = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::dp2(rods), 10);
    let rods = vec![1, 2];
    assert_eq!(Solution::dp2(rods), 0);
    let rods = vec![3, 4, 3, 3, 2];
    assert_eq!(Solution::dp2(rods), 6);
}
