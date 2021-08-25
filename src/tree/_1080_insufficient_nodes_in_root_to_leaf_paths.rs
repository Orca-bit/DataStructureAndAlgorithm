use super::util::*;

struct Solution;

impl Solution {
    pub fn sufficient_subset(root: TreeLink, limit: i32) -> TreeLink {
        Self::postorder(root, limit)
    }

    fn postorder(root: TreeLink, limit: i32) -> TreeLink {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if left.is_none() && right.is_none() {
                if val >= limit {
                    Some(node)
                } else {
                    None
                }
            } else {
                let new_left = Self::postorder(left, limit - val);
                let new_right = Self::postorder(right, limit - val);
                if new_left.is_some() || new_right.is_some() {
                    node.borrow_mut().left = new_left;
                    node.borrow_mut().right = new_right;
                    Some(node)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(
            2,
            tree!(4, tree!(8), tree!(9)),
            tree!(-99, tree!(-99), tree!(-99))
        ),
        tree!(
            3,
            tree!(-99, tree!(12), tree!(13)),
            tree!(7, tree!(-99), tree!(14))
        )
    );
    let limit = 1;
    let res = tree!(
        1,
        tree!(2, tree!(4, tree!(8), tree!(9)), None),
        tree!(3, None, tree!(7, None, tree!(14)))
    );
    assert_eq!(Solution::sufficient_subset(root, limit), res);
    let root = tree!(
        5,
        tree!(4, tree!(11, tree!(7), tree!(1)), None),
        tree!(8, tree!(17), tree!(4, tree!(5), tree!(3)))
    );
    let limit = 22;
    let res = tree!(
        5,
        tree!(4, tree!(11, tree!(7), None), None),
        tree!(8, tree!(17), tree!(4, tree!(5), None))
    );
    assert_eq!(Solution::sufficient_subset(root, limit), res);
    let root = tree!(1, tree!(2, tree!(-5), None), tree!(-3, tree!(4), None));
    let limit = -1;
    let res = tree!(1, None, tree!(-3, tree!(4), None));
    assert_eq!(Solution::sufficient_subset(root, limit), res);
}
