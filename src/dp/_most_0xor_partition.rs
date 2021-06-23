use std::cmp::max;
use std::collections::HashMap;

fn most_partition(nums: Vec<i32>) -> i32 {
    let mut xor = 0;
    let mut dp = vec![0; nums.len()];
    let mut map = HashMap::new();
    map.insert(0, -1);
    for i in 0..nums.len() {
        xor ^= nums[i];
        if let Some(pre) = map.insert(xor, i as i32) {
            dp[i] = if pre == -1 { 1 } else { dp[pre as usize] + 1 };
        }
        if i > 0 {
            dp[i] = max(dp[i], dp[i - 1]);
        }
    }
    *dp.last().unwrap()
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1, 0, 1, 2, 3];
    assert_eq!(most_partition(nums), 3);
}
