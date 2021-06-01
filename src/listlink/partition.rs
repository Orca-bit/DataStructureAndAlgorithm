use super::util::*;

struct Solution;

impl Solution {
    fn partition(mut head: ListLink, x: i32) -> ListLink {
        let mut left = vec![];
        let mut mid = vec![];
        let mut right = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                left.push(Some(node));
            } else if node.val > x {
                right.push(Some(node));
            } else {
                mid.push(Some(node));
            }
        }
        let mut res = None;
        while let Some(link) = right.pop() {
            let mut node = link.unwrap();
            node.next = res;
            res = Some(node);
        }
        while let Some(link) = mid.pop() {
            let mut node = link.unwrap();
            node.next = res;
            res = Some(node);
        }
        while let Some(link) = left.pop() {
            let mut node = link.unwrap();
            node.next = res;
            res = Some(node);
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