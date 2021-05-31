use super::util::*;

struct Solution;

impl Solution {
    fn partition(head: ListLink, x: i32) -> ListLink {
        let mut p = head;
        let mut left = vec![];
        let mut mid = vec![];
        let mut right = vec![];
        while let Some(node) = p {
            if node.val < x {
                left.push(node.val);
            } else if node.val > x {
                right.push(node.val);
            } else {
                mid.push(node.val);
            }
            p = node.next;
        }
        let mut res = None;
        while let Some(val) = right.pop() {
            res = ListLink::link(val, res);
        }
        while let Some(val) = mid.pop() {
            res = ListLink::link(val, res);
        }
        while let Some(val) = left.pop() {
            res = ListLink::link(val, res);
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(1, 4, 3, 2, 5, 2);
    let x = 3;
    let res = list!(1, 2, 2, 3, 4, 5);
    assert_eq!(Solution::partition(head, x), res);
}