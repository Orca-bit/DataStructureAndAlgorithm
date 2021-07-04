use super::util::*;

struct Solution;

impl Solution {
    pub fn invert_tree(root: TreeLink) -> TreeLink {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Self::invert_tree(right);
            node.borrow_mut().right = Self::invert_tree(left);
            return Some(node);
        }
        None
    }
}

#[test]
fn test() {
    let input = tree!(
        4,
        tree!(2, tree!(1), tree!(3)),
        tree!(7, tree!(6), tree!(9))
    );
    let output = tree!(
        4,
        tree!(7, tree!(9), tree!(6)),
        tree!(2, tree!(3), tree!(1))
    );
    assert_eq!(Solution::invert_tree(input), output);
}
