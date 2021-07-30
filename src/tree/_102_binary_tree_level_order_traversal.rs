use super::util::*;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut deq = VecDeque::new();
        if let Some(node) = root {
            deq.push_back(Rc::clone(&node));
        }
        while !deq.is_empty() {
            let size = deq.len();
            let mut tmp = vec![];
            for _ in 0..size {
                let node = deq.pop_front().unwrap();
                let node = node.borrow();
                tmp.push(node.val);
                if let Some(ref node_left) = node.left {
                    deq.push_back(Rc::clone(node_left));
                }
                if let Some(ref node_right) = node.right {
                    deq.push_back(Rc::clone(node_right));
                }
            }
            res.push(tmp);
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<Vec<i32>> = vec_vec_i32![[3], [9, 20], [15, 7]];
    assert_eq!(Solution::level_order(root), res);
}
