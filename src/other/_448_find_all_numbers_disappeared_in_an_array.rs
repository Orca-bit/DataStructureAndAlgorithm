struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let n = nums.len();
        for i in 0..n {
            let index = nums[i].abs() as usize - 1;
            nums[index] = -nums[index].abs();
        }
        for i in 1..=n {
            if nums[i - 1] > 0 {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    assert_eq!(Solution::find_disappeared_numbers(nums), vec![5, 6]);
}