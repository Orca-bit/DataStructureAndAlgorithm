use super::util::*;

struct Solution;

impl Solution {
    pub fn find_bottom_left_value(root: TreeLink) -> i32 {
        Self::process(&root, 1).unwrap().most_left
    }

    fn process(root: &TreeLink, level: usize) -> Option<ReturnData> {
        if let Some(node) = root {
            let node = node.borrow();
            let left_data = Self::process(&node.left, level + 1);
            let right_data = Self::process(&node.right, level + 1);
            if left_data.is_none() && right_data.is_none() {
                return Some(ReturnData::new(node.val, level));
            }
            let mut most_left = 0;
            let mut level = 0;
            if let Some(data) = left_data {
                most_left = data.most_left;
                level = data.level;
            }
            if let Some(data) = right_data {
                if data.level > level {
                    most_left = data.most_left;
                    level = data.level;
                }
            }
            Some(ReturnData::new(most_left, level))
        } else {
            None
        }
    }
}

struct ReturnData {
    most_left: i32,
    level: usize,
}

impl ReturnData {
    fn new(most_left: i32, level: usize) -> Self {
        Self { most_left, level }
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(1), tree!(3));
    let res = 1;
    assert_eq!(Solution::find_bottom_left_value(root), res);
    let root = tree!(
        1,
        tree!(2, tree!(4), None),
        tree!(3, tree!(5, tree!(7), None), tree!(6))
    );
    let res = 7;
    assert_eq!(Solution::find_bottom_left_value(root), res);
}
