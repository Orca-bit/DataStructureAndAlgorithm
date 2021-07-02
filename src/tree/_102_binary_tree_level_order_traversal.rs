use super::util::*;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let core = || -> Option<Vec<Vec<i32>>> {
            let mut res = vec![];
            let mut queue = VecDeque::new();
            if root.is_some() {
                queue.push_back(root.clone());
            }
            while !queue.is_empty() {
                let mut cur = vec![];
                let size = queue.len();
                for _ in 0..size {
                    let head = queue.pop_front()?;
                    if head.is_some() {
                        cur.push(head.clone()?.borrow().val);
                        queue.push_back(head.clone()?.borrow().left.clone());
                        queue.push_back(head.clone()?.borrow().right.clone());
                    }
                }
                if !cur.is_empty() {
                    res.push(cur);
                }
            }
            Some(res)
        };
        core().unwrap()
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<Vec<i32>> = vec_vec_i32![[3], [9, 20], [15, 7]];
    assert_eq!(Solution::level_order(root), res);
}
