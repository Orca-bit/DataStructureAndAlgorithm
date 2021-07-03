use super::util::*;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeLink {
        let mut map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            map.insert(val, i);
        }
        Self::process(
            &preorder,
            0,
            preorder.len() as i32 - 1,
            &inorder,
            0,
            inorder.len() as i32 - 1,
            &map,
        )
    }

    fn process(
        preorder: &[i32],
        l1: i32,
        r1: i32,
        inorder: &[i32],
        l2: i32,
        r2: i32,
        map: &HashMap<i32, usize>,
    ) -> TreeLink {
        if l1 > r1 {
            return None;
        }
        let head = Rc::new(RefCell::new(TreeNode {
            val: preorder[l1 as usize],
            left: None,
            right: None,
        }));
        if l1 == r1 {
            return Some(head);
        }
        let find = *map.get(&preorder[l1 as usize])? as i32;
        head.borrow_mut().left =
            Self::process(preorder, l1 + 1, l1 + find - l2, inorder, l2, find - 1, map);
        head.borrow_mut().right =
            Self::process(preorder, l1 + find - l2 + 1, r1, inorder, find + 1, r2, map);
        Some(head)
    }
}


#[test]
fn test() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::build_tree(preorder, inorder), res);
}