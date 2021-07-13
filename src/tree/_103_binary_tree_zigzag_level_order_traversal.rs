use super::util::*;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: TreeLink) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut deq = VecDeque::new();
        let mut l2r = true;
        deq.push_back(root.clone());
        let mut res = vec![];
        while !deq.is_empty() {
            let size = deq.len();
            let mut cur = vec![];
            for _ in 0..size {
                let head = if l2r {
                    deq.pop_front().unwrap()
                } else {
                    deq.pop_back().unwrap()
                };
                if let Some(node) = head {
                    let node = node.borrow();
                    cur.push(node.val);
                    if l2r {
                        deq.push_back(node.left.clone());
                        deq.push_back(node.right.clone());
                    } else {
                        deq.push_front(node.right.clone());
                        deq.push_front(node.left.clone());
                    }
                }
            }
            if !cur.is_empty() {
                res.push(cur);
            }
            l2r = !l2r;
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = vec_vec_i32![[3], [20, 9], [15, 7]];
    assert_eq!(Solution::zigzag_level_order(root), res);
}