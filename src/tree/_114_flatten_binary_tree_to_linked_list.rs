use super::util::*;

struct Solution;

impl Solution {
    pub fn flatten(root: &mut TreeLink) {
        let mut pre = None;
        Self::postorder(root.take(), &mut pre);
        *root = pre;
    }

    // right left root
    fn postorder(root: TreeLink, pre: &mut TreeLink) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            Self::postorder(right, pre);
            Self::postorder(left, pre);
            node.borrow_mut().right = pre.take();
            *pre = Some(node);
        }
    }
}

#[test]
fn test() {
    let mut root = tree!(1, tree!(2, tree!(3), tree!(4)), tree!(5, None, tree!(6)));
    let res = tree!(
        1,
        None,
        tree!(
            2,
            None,
            tree!(3, None, tree!(4, None, tree!(5, None, tree!(6))))
        )
    );
    Solution::flatten(&mut root);
    assert_eq!(root, res);
}