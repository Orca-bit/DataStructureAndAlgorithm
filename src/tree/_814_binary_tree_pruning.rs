use super::util::*;

trait PostOrder {
    fn postorder(self) -> Self;
}

impl PostOrder for TreeLink {
    fn postorder(self) -> Self {
        if let Some(node) = self {
            let left = node.borrow_mut().left.take().postorder();
            let right = node.borrow_mut().right.take().postorder();
            if left.is_none() && right.is_none() && node.borrow().val == 0 {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.borrow().val,
                    left,
                    right,
                })))
            }
        } else {
            None
        }
    }
}

struct Solution;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.postorder()
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(0, tree!(0), tree!(1)));
    let res = tree!(1, None, tree!(0, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
    let root = tree!(
        1,
        tree!(0, tree!(0), tree!(0)),
        tree!(1, tree!(0), tree!(1))
    );
    let res = tree!(1, None, tree!(1, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
    let root = tree!(
        1,
        tree!(1, tree!(1, tree!(0), None), tree!(1)),
        tree!(0, tree!(0), tree!(1))
    );
    let res = tree!(1, tree!(1, tree!(1), tree!(1)), tree!(0, None, tree!(1)));
    assert_eq!(Solution::prune_tree(root), res);
}
