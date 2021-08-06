use super::util::*;
use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn distance_k(root: TreeLink, p: TreeLink, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut adj = HashMap::new();
        Self::dfs(root, &mut adj);
        let mut que = VecDeque::new();
        que.push_back((p.as_ref().unwrap().borrow().val, 0));
        let mut visited = HashSet::new();
        visited.insert(p.as_ref().unwrap().borrow().val);
        while let Some(x) = que.pop_front() {
            if x.1 == k {
                res.push(x.0);
            }
            for y in adj.entry(x.0).or_default() {
                if visited.insert(*y) {
                    que.push_back((*y, x.1 + 1));
                }
            }
        }
        res
    }

    fn dfs(root: TreeLink, adj: &mut HashMap<i32, Vec<i32>>) {
        if let Some(root) = root {
            let val = root.borrow().val;
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            if left.is_some() {
                let l_val = left.as_ref().unwrap().borrow().val;
                adj.entry(val).or_default().push(l_val);
                adj.entry(l_val).or_default().push(val);
                Self::dfs(left, adj);
            }
            if right.is_some() {
                let r_val = right.as_ref().unwrap().borrow().val;
                adj.entry(val).or_default().push(r_val);
                adj.entry(r_val).or_default().push(val);
                Self::dfs(right, adj);
            }
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = tree!(5);
    let k = 2;
    let mut res = vec![7, 4, 1];
    let mut ans = Solution::distance_k(root, p, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
