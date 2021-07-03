use super::util::*;
struct Solution;

impl Solution {
    pub fn max_path_sum(root: TreeLink) -> i32 {
        if let Some(data) = Self::process(&root) {
            return data.max_path;
        }
        0
    }

    fn process(root: &TreeLink) -> Option<Box<ReturnData>> {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            let left_data = Self::process(&node.left);
            let right_data = Self::process(&node.right);
            let mut p1 = i32::MIN;
            let mut p2 = i32::MIN;
            if let Some(ref data) = left_data {
                p1 = data.max_path;
                p2 = data.max_path_from_root + node.val;
            }
            let mut p3 = i32::MIN;
            let mut p4 = i32::MIN;
            if let Some(ref data) = right_data {
                p3 = data.max_path;
                p4 = data.max_path_from_root + node.val;
            }
            let p5 = node.val;
            let mut p6 = i32::MIN;
            if left_data.is_some() && right_data.is_some() {
                p6 = left_data?.max_path_from_root + right_data?.max_path_from_root + node.val;
            }
            let max_path = p1.max(p2).max(p3).max(p4).max(p5).max(p6);
            let max_path_from_root = p2.max(p4).max(p5);
            return Some(Box::new(ReturnData::new(max_path, max_path_from_root)));
        }
        None
    }
}

struct ReturnData {
    max_path: i32,
    max_path_from_root: i32,
}

impl ReturnData {
    fn new(max_path: i32, max_path_from_root: i32) -> Self {
        Self {
            max_path,
            max_path_from_root,
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    let res = 6;
    assert_eq!(Solution::max_path_sum(root), res);
    let root = tree!(-10, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = 42;
    assert_eq!(Solution::max_path_sum(root), res);
}