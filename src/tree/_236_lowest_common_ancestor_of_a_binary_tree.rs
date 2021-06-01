use super::util::*;
struct Solution;

impl Solution {
    fn lowest_common_ancestor(root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let mut res = None;
        root.find_lca(p_val, q_val, &mut res);
        res
    }
}

trait Recur {
    fn find_lca(&self, p_val: i32, q_val: i32, lca: &mut TreeLink) -> (bool, bool);
}

impl Recur for TreeLink {
    fn find_lca(&self, p_val: i32, q_val: i32, lca: &mut TreeLink) -> (bool, bool) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = node.borrow().left.find_lca(p_val, q_val, lca);
            let right= node.borrow().right.find_lca(p_val, q_val, lca);
            let res = (left.0 || right.0 || val == p_val, left.1 || right.1 || val == q_val);
            if lca.is_none() && res.0 && res.1 {
                *lca = Some(node.clone());
            }
            return res;
        }
        (false, false)
    }
}