use super::util::*;

struct Solution;

impl Solution {
    fn lowest_common_ancestor(root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let mut res = None;
        let _ = root.find_lca(p_val, q_val, &mut res);
        res
    }
}

trait Recur {
    fn find_lca(&self, p_val: i32, q_val: i32, lca: &mut TreeLink) -> ReturnData;
}

impl Recur for TreeLink {
    fn find_lca(&self, p_val: i32, q_val: i32, lca: &mut TreeLink) -> ReturnData {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = node.borrow().left.find_lca(p_val, q_val, lca);
            let right = node.borrow().right.find_lca(p_val, q_val, lca);
            let res = ReturnData::new(
                left.have_p || right.have_p || val == p_val,
                left.have_q || right.have_q || val == q_val,
            );
            if lca.is_none() && res.have_p && res.have_q {
                *lca = Some(Rc::clone(&node)); //第一次满足条件时设置为公共祖先，之后lca.is_some()
            }
            return res;
        }
        ReturnData::new(false, false)
    }
}

struct ReturnData {
    have_p: bool,
    have_q: bool,
}

impl ReturnData {
    fn new(have_p: bool, have_q: bool) -> Self {
        Self { have_p, have_q }
    }
}
