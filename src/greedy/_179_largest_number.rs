struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| (b.clone() + a.as_str()).cmp(&(a.clone() + b.as_str())));
        if nums[0] == "0" {
            return "0".to_string();
        }
        let mut res = "".to_string();
        for s in nums {
            res += s.as_str();
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![10, 2];
    let res = "210".to_string();
    assert_eq!(Solution::largest_number(nums), res);
    let nums = vec![3, 30, 34, 5, 9];
    let res = "9534330".to_string();
    assert_eq!(Solution::largest_number(nums), res);
    let nums = vec![0, 0];
    let res = "0".to_string();
    assert_eq!(Solution::largest_number(nums), res);
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = "9876543210".to_string();
    assert_eq!(Solution::largest_number(nums), res);
}
