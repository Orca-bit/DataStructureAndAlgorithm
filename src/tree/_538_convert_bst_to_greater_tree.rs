use super::util::*;

struct Solution;

impl Solution {
    pub fn convert_bst(root: TreeLink) -> TreeLink {
        Self::pos_inorder(&root, &mut 0);
        root
    }

    fn pos_inorder(root: &TreeLink, pre: &mut i32) {
        if let Some(node) = root {
            Self::pos_inorder(&node.borrow().right, pre);
            *pre += node.borrow().val;
            node.borrow_mut().val = *pre;
            Self::pos_inorder(&node.borrow().left, pre);

        }
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(2), tree!(13));
    let greater = tree!(18, tree!(20), tree!(13));
    assert_eq!(Solution::convert_bst(root), greater);
}