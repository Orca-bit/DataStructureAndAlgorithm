struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![-1; arr.len() + 1];
        Self::process(&arr, arr.len(), k as usize, &mut dp)
    }

    fn process(arr: &[i32], index: usize, k: usize, dp: &mut Vec<i32>) -> i32 {
        if dp[index] != -1 {
            return dp[index];
        }
        if index == 0 {
            return 0;
        }
        let mut res = 0;
        let mut max = arr[index - 1];
        for i in 0..k.min(index) {
            let pre = Self::process(arr, index - i - 1, k, dp);
            max = max.max(arr[index - i - 1]);
            res = res.max(pre + max * (i + 1) as i32);
        }
        dp[index] = res;
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 15, 7, 9, 2, 5, 10];
    let k = 3;
    let res = 84;
    assert_eq!(Solution::max_sum_after_partitioning(a, k), res);
}
