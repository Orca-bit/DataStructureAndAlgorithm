struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort_unstable();
        let mut index = 0;
        while index < nums.len() - 1 {
            nums.swap(index, index + 1);
            index += 2;
        }
    }
}

#[test]
fn test() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::wiggle_sort(&mut nums);
    println!("{:?}", nums);
}
