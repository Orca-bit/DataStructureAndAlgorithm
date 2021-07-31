struct Solution;

impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let sum = cost.iter().sum::<i32>();
        let mut dp = vec![0; 26];
        let mut max = 0;
        for i in 0..n {
            let index = (s[i] - b'a') as usize;
            dp[index] = dp[index].max(
                (0..26)
                    .filter(|&other| other != index)
                    .map(|other| dp[other])
                    .max()
                    .unwrap()
                    + cost[i],
            );
            max = max.max(dp[index]);
        }
        sum - max
    }
}

#[test]
fn test() {
    let s = "abaac".to_string();
    let cost = vec![1, 2, 3, 4, 5];
    let res = 3;
    assert_eq!(Solution::min_cost(s, cost), res);
    let s = "baab".to_string();
    let cost = vec![8, 7, 2, 10];
    let res = 2;
    assert_eq!(Solution::min_cost(s, cost), res);
}
