use super::util::*;
use std::cmp::min;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_depth_1(root: TreeLink) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut que = VecDeque::new();
        que.push_back((root.clone(), 1));
        while let Some((link, depth)) = que.pop_front() {
            if let Some(node) = link {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => return depth,
                    (Some(_), None) => que.push_back((node.left.clone(), depth + 1)),
                    (None, Some(_)) => que.push_back((node.right.clone(), depth + 1)),
                    (Some(_), Some(_)) => {
                        que.push_back((node.left.clone(), depth + 1));
                        que.push_back((node.right.clone(), depth + 1));
                    }
                }
            }
        }
        0
    }

    fn min_depth(root: TreeLink) -> i32 {
        Self::process(&root) as i32
    }

    fn process(root: &TreeLink) -> usize {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (_, _) => min(Self::process(&node.left), Self::process(&node.right)) + 1,
                }
            }
        }
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::min_depth(root), 2);
}
