use super::util::*;

struct Solution;

impl Solution {
    pub fn max_depth(root: TreeLink) -> i32 {
        fn core(root: TreeLink) -> Option<i32> {
            if root.is_none() {
                return Some(0);
            }
            if root.clone()?.borrow().left.is_none() && root.clone()?.borrow().right.is_none() {
                return Some(1);
            }
            Some(
                core(root.clone()?.borrow().left.clone())?
                    .max(core(root.clone()?.borrow().right.clone())?) + 1
            )
        }
        core(root).unwrap()
    }
}

#[test]
fn test() {
    let p = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::max_depth(p), 3);
}