use super::util::*;

struct Solution;

impl Solution {
    pub fn merge_trees(root1: TreeLink, root2: TreeLink) -> TreeLink {
        match (root1, root2) {
            (None, None) => None,
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(node1), Some(node2)) => {
                let mut merge = Rc::new(RefCell::new(TreeNode {
                    val: node1.borrow().val + node2.borrow().val,
                    left: None,
                    right: None,
                }));
                merge.borrow_mut().left = Self::merge_trees(
                    node1.borrow_mut().left.take(),
                    node2.borrow_mut().left.take(),
                );
                merge.borrow_mut().right = Self::merge_trees(
                    node1.borrow_mut().right.take(),
                    node2.borrow_mut().right.take(),
                );
                Some(merge)
            }
        }
    }
}

#[test]
fn test() {
    let t1 = tree!(1, tree!(3, tree!(5), None), tree!(2));
    let t2 = tree!(2, tree!(1, None, tree!(4)), tree!(3, None, tree!(7)));
    let res = tree!(3, tree!(4, tree!(5), tree!(4)), tree!(5, None, tree!(7)));
    assert_eq!(Solution::merge_trees(t1, t2), res);
}
