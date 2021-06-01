use super::util::*;

struct Solution;

impl Solution {
    fn is_balanced(root: TreeLink) -> bool {
        let data = root.is_balance_tree();
        data.is_balance
    }
}

trait Recur {
    fn is_balance_tree(&self) -> ReturnData;
}

impl Recur for TreeLink {
    fn is_balance_tree(&self) -> ReturnData {
        if let Some(node) = self {
            let node = node.borrow();
            let left_data = node.left.is_balance_tree();
            let right_data = node.right.is_balance_tree();
            return ReturnData::new(
                left_data.is_balance
                    && right_data.is_balance
                    && (left_data.height - right_data.height).abs() <= 1,
                isize::max(left_data.height, right_data.height) + 1,
            );
        }
        ReturnData::new(true, 0)
    }
}

struct ReturnData {
    is_balance: bool,
    height: isize,
}

impl ReturnData {
    fn new(is_balance: bool, height: isize) -> Self {
        Self { is_balance, height }
    }
}

#[test]
fn test() {
    let a = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::is_balanced(a), true);
    let b = tree!(
        1,
        tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
        tree!(2)
    );
    assert_eq!(Solution::is_balanced(b), false);
}
