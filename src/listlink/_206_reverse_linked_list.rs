struct Solution;
use super::util::*;

impl Solution {
    fn reverse_list(head: ListLink) -> ListLink {
        let mut p = head;
        let mut pre = None;
        while let Some(mut node) = p.take() {
            p = node.next;
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
}

#[test]
fn test() {
    let head = list!();
    let res = list!();
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1);
    let res = list!(1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1, 2);
    let res = list!(2, 1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1, 2, 3);
    let res = list!(3, 2, 1);
    assert_eq!(Solution::reverse_list(head), res);
}