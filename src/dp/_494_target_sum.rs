use std::mem::swap;

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum < target || (sum & 1) != (target & 1) {
            return 0;
        }
        let positive_sum = (target + sum) as usize >> 1;
        let n = nums.len();
        let mut dp1 = vec![0; positive_sum + 1];
        let mut dp2 = vec![0; positive_sum + 1];
        dp1[0] = 1;
        for i in (0..n).rev() {
            swap(&mut dp1, &mut dp2);
            for j in 0..=positive_sum {
                dp1[j] = dp2[j]
                    + if j >= nums[i] as usize {
                        dp2[j - nums[i] as usize]
                    } else {
                        0
                    };
            }
        }
        dp1[positive_sum]
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 1, 1];
    let s = 3;
    let res = 5;
    assert_eq!(Solution::find_target_sum_ways(nums, s), res);
}