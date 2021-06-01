use super::util::*;

struct Solution;

impl Solution {
    fn is_valid_bst_recur(root: TreeLink) -> bool { // 左神套路解法
        if let Some(data) = root.is_bst_recur() {
            return data.is_bst;
        }
        true
    }

    fn is_valid_bst(root: TreeLink) -> bool { //rustgym 解法
        let mut pre = None;
        let mut res = true;
        root.inorder(&mut |x| {
            if let Some(y) = pre {
                if x <= y {
                    res = false;
                    return;
                }
            }
            pre = Some(x);
        });
        res
    }
}

trait Inorder {
    fn inorder(&self, action: &mut dyn FnMut(i32));
}

impl Inorder for TreeLink {
    fn inorder(&self, action: &mut dyn FnMut(i32)) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.inorder(action);
            action(node.val);
            node.right.inorder(action);
        }
    }
}

trait Recur {
    fn is_bst_recur(&self) -> Option<ReturnData>;
}

impl Recur for TreeLink {
    fn is_bst_recur(&self) -> Option<ReturnData> {
        if let Some(node) = self {
            let node = node.borrow();
            let mut min = node.val;
            let mut max = node.val;
            let mut is_bst = true;
            if let Some(left_data) = node.left.is_bst_recur() {
                min = min.min(left_data.min);
                max = max.max(left_data.max);
                if !left_data.is_bst || left_data.max >= node.val {
                    is_bst = false;
                }
            }
            if let Some(right_data) = node.right.is_bst_recur() {
                min = min.min(right_data.min);
                max = max.max(right_data.max);
                if !right_data.is_bst || right_data.min <= node.val {
                    is_bst = false;
                }
            }
            return Some(ReturnData::new(is_bst, min, max));
        }
        None
    }
}

struct ReturnData {
    is_bst: bool,
    min: i32,
    max: i32
}

impl ReturnData {
    fn new(is_bst: bool, min: i32, max: i32) -> Self {
        Self {
            is_bst,
            min,
            max
        }
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(1), tree!(3));
    assert_eq!(Solution::is_valid_bst_recur(root), true);
    let root = tree!(5, tree!(1), tree!(4, tree!(3), tree!(6)));
    assert_eq!(Solution::is_valid_bst_recur(root), false);
}