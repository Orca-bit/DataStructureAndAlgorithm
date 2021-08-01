use crate::tree::util::*;

struct Solution;

impl Solution {
    pub fn longest_univalue_path(root: TreeLink) -> i32 {
        if root.is_some() {
            Self::process(&root).unwrap().max - 1
        } else {
            0
        }
    }

    fn process(root: &TreeLink) -> Option<Box<ReturnData>> {
        if let Some(node) = root {
            let node = node.borrow();
            let mut p1 = 1;
            let mut p2 = 1;
            if let Some(left_data) = Self::process(&node.left) {
                p1 = left_data.max;
                if let Some(left) = node.left.as_ref() {
                    if node.val == left.borrow().val {
                        p2 = left_data.have + 1;
                    }
                }
            }
            let mut p3 = 1;
            let mut p4 = 1;
            if let Some(right_data) = Self::process(&node.right) {
                p3 = right_data.max;
                if let Some(right) = node.right.as_ref() {
                    if node.val == right.borrow().val {
                        p4 = right_data.have + 1;
                    }
                }
            }
            let max = 1.max(p1).max(p3).max(p2 + p4 - 1);
            let have = 1.max(p2).max(p4);
            Some(Box::new(ReturnData::new(max, have)))
        } else {
            None
        }
    }
}

struct ReturnData {
    max: i32,
    have: i32,
}

impl ReturnData {
    pub fn new(max: i32, have: i32) -> Self {
        Self { max, have }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(4, tree!(4), tree!(4)), tree!(5, None, tree!(5)));
    assert_eq!(Solution::longest_univalue_path(root), 2);
    let root = tree!(3, tree!(3), None);
    assert_eq!(Solution::longest_univalue_path(root), 1);
}
