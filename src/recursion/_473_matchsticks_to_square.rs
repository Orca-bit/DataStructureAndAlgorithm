struct Solution;

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let n = matchsticks.len();
        if n < 4 {
            return false;
        }
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }
        let mut sides = vec![0; 4];
        matchsticks.sort_unstable_by(|a, b| b.cmp(a));
        Self::dfs(0, matchsticks.as_slice(), &mut sides, sum >> 2)
    }

    fn dfs(index: usize, nums: &[i32], sides: &mut Vec<i32>, length: i32) -> bool {
        if index == nums.len() {
            true
        } else {
            for i in 0..4 {
                if sides[i] + nums[index] > length {
                    continue;
                }
                sides[i] += nums[index];
                if Self::dfs(index + 1, nums, sides, length) {
                    return true;
                }
                sides[i] -= nums[index];
            }
            false
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 2];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
    let nums = vec![1, 1, 2, 2, 2];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
    let nums = vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1]), true);
}
