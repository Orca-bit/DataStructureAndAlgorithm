use super::util::*;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn path_sum(root: TreeLink, target: i32) -> i32 {
        let mut pre_map = HashMap::new();
        pre_map.insert(0, 1);
        Self::process(&root, 0, target, &mut pre_map)
    }

    fn process(root: &TreeLink, pre_sum: i32, target: i32, pre_map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let mut res = 0;
            let sum = pre_sum + node.val;
            if let Some(&ans) = pre_map.get(&(sum - target)) {
                res += ans;
            }
            *pre_map.entry(sum).or_default() += 1;
            res += Self::process(&node.left, sum, target, pre_map);
            res += Self::process(&node.right, sum, target, pre_map);
            if *pre_map.get(&sum).unwrap() == 1 {
                pre_map.remove(&sum);
            } else {
                *pre_map.entry(sum).or_insert(1) -= 1;
            }
            res
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        10,
        tree!(5, tree!(3, tree!(3), tree!(-2)), tree!(2, None, tree!(1))),
        tree!(-3, None, tree!(11))
    );
    assert_eq!(Solution::path_sum(root, 8), 3);
}
