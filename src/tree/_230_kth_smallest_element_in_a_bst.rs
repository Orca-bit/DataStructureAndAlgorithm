use super::util::*;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: TreeLink, k: i32) -> i32 {
        let mut count = 0;
        let mut res = 0;
        Self::inorder(&root, k, &mut count, &mut res);
        res
    }

    fn inorder(root: &TreeLink, k: i32, count: &mut i32, res: &mut i32) {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            Self::inorder(&node.left, k, count, res);
            *count += 1;
            if *count == k {
                *res = node.val;
                return;
            }
            Self::inorder(&node.right, k, count, res);
        }
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(1, None, tree!(2)), tree!(4));
    let k = 1;
    let res = 1;
    assert_eq!(Solution::kth_smallest(root, k), res);
    let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
    let k = 3;
    let res = 3;
    assert_eq!(Solution::kth_smallest(root, k), res);
}
