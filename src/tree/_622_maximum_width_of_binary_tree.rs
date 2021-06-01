use super::util::*;
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn width_of_binary_tree(root: TreeLink) -> i32 {
        let mut min= HashMap::new();
        let mut max= HashMap::new();
        let mut res = 0;
        root.preorder(0, 0, &mut min, &mut max, &mut res);
        res as i32
    }
}

trait Preorder {
    fn preorder(
        &self,
        pos: u32, // 当前节点在所在层的位置
        level: u32, // 所在层
        min_pos_hash: &mut HashMap<u32, u32>, // 层 -> 最小位置
        max_pos_hash: &mut HashMap<u32, u32>, // 层 -> 最大位置
        max_diff: &mut u32 // 位置差
    );
}

impl Preorder for TreeLink {
    fn preorder(
        &self,
        pos: u32,
        level: u32,
        min_pos_hash: &mut HashMap<u32, u32>,
        max_pos_hash: &mut HashMap<u32, u32>,
        max_diff: &mut u32
    ) {
        if let Some(node) = self {
            min_pos_hash.entry(level).or_insert(pos);
            max_pos_hash.entry(level).or_insert(pos);
            *min_pos_hash.get_mut(&level).unwrap() = min_pos_hash[&level].min(pos);
            *max_pos_hash.get_mut(&level).unwrap() = max_pos_hash[&level].max(pos);
            *max_diff = (*max_diff).max(max_pos_hash[&level] - min_pos_hash[&level] + 1);
            let node = node.borrow();
            node.left.preorder(pos << 1, level + 1, min_pos_hash, max_pos_hash, max_diff);
            node.right.preorder((pos << 1) + 1, level + 1, min_pos_hash, max_pos_hash, max_diff);
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(3, tree!(5), tree!(3)), tree!(2, None, tree!(9)));
    let res = 4;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(1, tree!(3, tree!(5), tree!(3)), None);
    let res = 2;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(1, tree!(3, tree!(5), None), tree!(2));
    let res = 2;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(
        1,
        tree!(3, tree!(5, tree!(6), None), None),
        tree!(2, None, tree!(9, None, tree!(7)))
    );
    let res = 8;
    assert_eq!(Solution::width_of_binary_tree(root), res);
}