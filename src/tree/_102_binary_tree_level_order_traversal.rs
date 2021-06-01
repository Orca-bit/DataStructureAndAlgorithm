use super::util::*;
struct Solution;

impl Solution {
    fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut res = vec![];
        root.preorder(0, &mut res);
        res
    }
}

trait Preorder {
    fn preorder(&self, level: usize, v: &mut Vec<Vec<i32>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, level: usize, v: &mut Vec<Vec<i32>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if v.len() == level {
                v.push(vec![val]);
            } else {
                v[level].push(val);
            }
            node.left.preorder(level + 1, v);
            node.right.preorder(level + 1, v);
        }
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<Vec<i32>> = vec_vec_i32![[3], [9, 20], [15, 7]];
    assert_eq!(Solution::level_order(root), res);
}