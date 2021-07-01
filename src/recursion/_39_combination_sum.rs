struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::process(&candidates, 0, target, &mut res, &mut path);
        res
    }

    fn process(
        candidates: &[i32],
        index: usize,
        target: i32,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if target == 0 {
            res.push(path.clone());
            return;
        }
        if index == candidates.len() {
            return;
        }
        let mut i = 0;
        while candidates[index] * i <= target {
            for _ in 0..i {
                path.push(candidates[index]);
            }
            Self::process(
                candidates,
                index + 1,
                target - candidates[index] * i,
                res,
                path,
            );
            for _ in 0..i {
                path.pop();
            }
            i += 1;
        }
    }
}

#[test]
fn test() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let res = vec![vec![7], vec![2, 2, 3]];
    assert_eq!(Solution::combination_sum(candidates, target), res);
    let candidates = vec![2, 3, 5];
    let target = 8;
    let res = vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]];
    assert_eq!(Solution::combination_sum(candidates, target), res);
}