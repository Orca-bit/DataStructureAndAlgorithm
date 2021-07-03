struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut hp = 1;
        for &num in nums.iter().skip(1) {
            if hp == 0 {
                res = num;
                hp = 1;
            } else if res == num {
                hp += 1;
            } else {
                hp -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(nums), 2);
    let nums = vec![3, 3, 4];
    assert_eq!(Solution::majority_element(nums), 3);
}