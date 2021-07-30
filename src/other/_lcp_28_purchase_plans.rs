struct Solution;

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn purchase_plans(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            if nums[l] + nums[r] <= target {
                res += r - l;
                res %= MOD;
                l += 1;
            } else {
                r -= 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::purchase_plans(vec![2, 5, 3, 5], 6), 1);
}
