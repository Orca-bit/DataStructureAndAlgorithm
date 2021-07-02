struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::process(&nums, 0, &mut res, &mut path);
        res
    }

    fn process(nums: &Vec<i32>, index: usize, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
        if index == nums.len() {
            res.push(path.clone());
            return;
        }
        Self::process(nums, index + 1, res, path);
        path.push(nums[index]);
        Self::process(nums, index + 1, res, path);
        path.pop();
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = vec![
        vec![],
        vec![3],
        vec![2],
        vec![2, 3],
        vec![1],
        vec![1, 3],
        vec![1, 2],
        vec![1, 2, 3],
    ];
    assert_eq!(Solution::subsets(nums), res);
}
