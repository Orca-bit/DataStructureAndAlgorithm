struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = vec![];
        for i in (2..nums.len()).rev() {
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                let mut next = Self::two_sum(&nums[0..i], -nums[i]);
                for mut cur in next {
                    cur.push(nums[i]);
                    res.push(cur);
                }
            }
        }
        res
    }

    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut res = vec![];
        while l < r {
            if nums[l] + nums[r] < target {
                l += 1;
            } else if nums[l] + nums[r] > target {
                r -= 1;
            } else {
                if l == 0 || nums[l] != nums[l - 1] {
                    let mut cur = vec![nums[l], nums[r]];
                    res.push(cur);
                }
                l += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[-1, -1, 2], [-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), res);
    let nums = vec![-2, 0, 1, 1, 2];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[-2, 0, 2], [-2, 1, 1]];
    assert_eq!(Solution::three_sum(nums), res);
}