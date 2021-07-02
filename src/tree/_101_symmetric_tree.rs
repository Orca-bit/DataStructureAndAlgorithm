use super::util::*;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: TreeLink) -> bool {
        if let Some(res) = Self::process(root.clone(), root.clone()) {
            return res;
        }
        false
    }

    fn process(root1: TreeLink, root2: TreeLink) -> Option<bool> {
        if root1.is_none() && root2.is_none() {
            return Some(true);
        }
        if root1.is_some() && root2.is_some() {
            return Some(
                root1.clone()?.borrow().val == root2.clone()?.borrow().val
                    && Self::process(
                        root1.clone()?.borrow().left.clone(),
                        root2.clone()?.borrow().right.clone(),
                    )?
                    && Self::process(
                        root1.clone()?.borrow().right.clone(),
                        root2.clone()?.borrow().left.clone(),
                    )?,
            );
        }
        Some(false)
    }
}

#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    assert_eq!(Solution::is_symmetric(q), true)
}
