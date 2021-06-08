use super::util::*;
use std::cmp::max;

struct ReturnData {
    max_distance: i32,
    height: i32,
}

impl ReturnData {
    fn new(max_distance: i32, height: i32) -> Self {
        Self {
            max_distance,
            height,
        }
    }
}

fn process(root: &TreeLink) -> ReturnData {
    if let Some(node) = root {
        let node = node.borrow();
        let left = process(&node.left);
        let right = process(&node.right);
        let height = max(left.height, right.height) + 1;
        let max_distance = max(
            max(left.max_distance, right.max_distance),
            left.height + right.height + 1,
        );
        let res = ReturnData::new(max_distance, height);
        return res;
    }
    ReturnData::new(0, 0)
}
