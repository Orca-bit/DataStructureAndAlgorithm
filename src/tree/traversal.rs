use super::util::*;

struct Solution;

impl Solution {
    fn preorder_traversal_recur(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        fn core(root: TreeLink, v: &mut Vec<i32>) {
            if let Some(node) = root {
                v.push(node.borrow().val);
                core(node.borrow_mut().left.take(), v);
                core(node.borrow_mut().right.take(), v);
            }
        }
        core(root, &mut res);
        res
    }

    fn preorder_traversal_unrecur(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        let mut stk = vec![root];
        while let Some(peek) = stk.pop() {
            if let Some(node) = peek {
                res.push(node.borrow().val);
                stk.push(node.borrow_mut().right.take());
                stk.push(node.borrow_mut().left.take());
            }
        }
        res
    }

    fn inorder_traversal_recur(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        fn core(root: TreeLink, v: &mut Vec<i32>) {
            if let Some(node) = root {
                core(node.borrow_mut().left.take(), v);
                v.push(node.borrow().val);
                core(node.borrow_mut().right.take(), v);
            }
        }
        core(root, &mut res);
        res
    }

    fn inorder_traversal_unrecur(mut root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        let mut stk = vec![];
        while !stk.is_empty() || root.is_some() {
           if let Some(node) = root {
               root = node.borrow_mut().left.take();
               stk.push(Some(node));
           } else {
               let peek = stk.pop().unwrap();
               if let Some(node) = peek {
                   res.push(node.borrow().val);
                   root = node.borrow_mut().right.take();
               }
           }
        }
        res
    }

    fn postorder_traversal_recur(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        fn core(root: TreeLink, v: &mut Vec<i32>) {
            if let Some(node) = root {
                core(node.borrow_mut().left.take(), v);
                core(node.borrow_mut().right.take(), v);
                v.push(node.borrow().val);
            }
        }
        core(root, &mut res);
        res
    }

    fn postorder_traversal_unrecur(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        let mut stk1 = vec![root];
        let mut stk2 = vec![];
        while let Some(peek) = stk1.pop() {
            if let Some(node) = peek {
                stk1.push(node.borrow_mut().left.take());
                stk1.push(node.borrow_mut().right.take());
                stk2.push(Some(node));
            }
        }
        while let Some(peek) = stk2.pop() {
            if let Some(node) = peek {
                res.push(node.borrow().val);
            }
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    let res = vec![1, 2, 3];
    assert_eq!(Solution::preorder_traversal_unrecur(root), res);
}