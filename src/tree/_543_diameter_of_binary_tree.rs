use super::util::*;

struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: TreeLink) -> i32 {
        Self::process(&root).diameter
    }

    fn process(root: &TreeLink) -> ReturnData {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::process(&node.left);
            let right = Self::process(&node.right);
            let height = left.height.max(right.height) + 1;
            let diameter = left
                .diameter
                .max(right.diameter)
                .max(left.height + right.height);
            ReturnData::new(diameter, height)
        } else {
            ReturnData::new(0, 0)
        }
    }
}

struct ReturnData {
    diameter: i32,
    height: i32,
}

impl ReturnData {
    fn new(diameter: i32, height: i32) -> Self {
        Self { diameter, height }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}
