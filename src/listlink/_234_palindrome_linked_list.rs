use super::util::*;

struct Solution;

impl Solution {
    fn is_palindrome(head: &ListLink) -> bool {
        let mut v = vec![];
        let mut fast_p = head;
        let mut slow_p = head;
        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            slow_p = &slow_p.as_ref().unwrap().next;
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        while let Some(node) = slow_p {
            v.push(node.val);
            slow_p = &node.next;
        }
        let mut p = head;
        while let Some(item) = v.pop() {
            if p.as_ref().unwrap().val != item {
                return false;
            }
            p = &p.as_ref().unwrap().next;
        }
        true
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 2, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1, 2, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1, 2, 3, 2, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1);
    assert_eq!(Solution::is_palindrome(&head), true);
}