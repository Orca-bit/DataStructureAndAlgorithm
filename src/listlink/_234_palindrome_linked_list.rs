use super::util::*;

struct Solution;

impl Solution {
    fn is_palindrome(head: &ListLink) -> bool {
        fn is_palindrome(head: &ListLink) -> Option<bool> {
            let mut v = vec![];
            let mut fast_p = head.as_ref();
            let mut slow_p = head.as_ref();
            while fast_p.is_some() && fast_p?.next.is_some() {
                slow_p = slow_p?.next.as_ref();
                fast_p = fast_p?.next.as_ref()?.next.as_ref();
            }
            while let Some(node) = slow_p {
                v.push(node.val);
                slow_p = node.next.as_ref();
            }
            let mut p = head.as_ref();
            while let Some(item) = v.pop() {
                if p?.val != item {
                    return Some(false);
                }
                p = p?.next.as_ref();
            }
            Some(true)
        }
        is_palindrome(head).unwrap_or_default()
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 1, 1);
    assert_eq!(Solution::is_palindrome(&head), false);
    let head = list!(1, 2, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1, 2, 3, 2, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1, 1);
    assert_eq!(Solution::is_palindrome(&head), true);
    let head = list!(1);
    assert_eq!(Solution::is_palindrome(&head), true);
}
