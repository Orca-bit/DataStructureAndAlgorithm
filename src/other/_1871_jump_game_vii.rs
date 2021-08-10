struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut iter = s.chars().skip(min_jump as usize);
        let n = s.len();
        let mut dp = vec![0; n];
        let mut pre = vec![1; n];
        dp[0] = 1;
        for i in min_jump as usize..n {
            let left = 0.max(i as i32 - max_jump) as usize;
            let right = i - min_jump as usize;
            let sum = pre[right] - if left == 0 { 0 } else { pre[left - 1] };
            let next_c = iter.next().unwrap();
            if sum > 0 && next_c == '0' {
                dp[i] = 1;
            }
            pre[i] = pre[i - 1] + dp[i];
        }
        dp[n - 1] == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_reach("011010".to_string(), 2, 3), true);
    assert_eq!(Solution::can_reach("01101110".to_string(), 2, 3), false);
    assert_eq!(Solution::can_reach("0000000000".to_string(), 2, 5), true);
    let s = ['0'; 100000].into_iter().collect();
    assert_eq!(Solution::can_reach(s, 5, 99998), true);
}
