use super::util::*;

struct Solution;

impl Solution {
    pub fn rob(root: TreeLink) -> i32 {
        let data = Self::process(&root);
        data.yes.max(data.no)
    }

    fn process(node: &TreeLink) -> ReturnData {
        if let Some(head) = node {
            let head = head.borrow();
            let left = Self::process(&head.left);
            let right = Self::process(&head.right);
            let yes = left.no + right.no + head.val;
            let no = left.no.max(left.yes) + right.no.max(right.yes);
            ReturnData::new(yes, no)
        } else {
            ReturnData::new(0, 0)
        }
    }
}

struct ReturnData {
    yes: i32,
    no: i32,
}

impl ReturnData {
    fn new(yes: i32, no: i32) -> Self {
        Self { yes, no }
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(2, None, tree!(3)), tree!(3, None, tree!(1)));
    let res = 7;
    assert_eq!(Solution::rob(root), res);
    let root = tree!(3, tree!(4, tree!(1), tree!(3)), tree!(5, None, tree!(1)));
    let res = 9;
    assert_eq!(Solution::rob(root), res);
}