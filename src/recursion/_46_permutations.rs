struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return Vec::new();
        }
        let mut res = vec![];
        let mut path = vec![];
        Self::process(&mut nums, 0, &mut res, &mut path);
        res
    }

    fn process(nums: &mut Vec<i32>, index: usize, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
        if index == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in index..nums.len() {
            nums.swap(index, i);
            path.push(nums[index]);
            Self::process(nums, index + 1, res, path);
            path.pop();
            nums.swap(index, i);
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let mut ans: Vec<Vec<i32>> = [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    let mut res = Solution::permute(nums);
    ans.sort_unstable();
    res.sort_unstable();
    assert_eq!(res, ans);
}
