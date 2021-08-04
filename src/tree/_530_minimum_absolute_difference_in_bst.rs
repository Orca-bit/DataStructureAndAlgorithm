use crate::tree::util::*;

struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: TreeLink) -> i32 {
        let mut res = i32::MAX;
        let mut pre = None;
        Self::inorder(&mut res, &mut pre, &root);
        res
    }

    fn inorder(res: &mut i32, pre: &mut Option<i32>, root: &TreeLink) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(res, pre, &node.left);
            if let Some(val) = pre.as_mut() {
                *res = (*res).min((node.val - *val).abs());
                *val = node.val;
            } else {
                *pre = Some(node.val);
            }
            Self::inorder(res, pre, &node.right);
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(3, tree!(2), None));
    assert_eq!(Solution::get_minimum_difference(root), 1);
}
