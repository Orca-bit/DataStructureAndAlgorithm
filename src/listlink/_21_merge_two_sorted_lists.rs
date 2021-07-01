use super::util::*;
struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: ListLink, l2: ListLink) -> ListLink {
        match (l1, l2) {
            (None, None) => None,
            (Some(head1), None) => Some(head1),
            (None, Some(head2)) => Some(head2),
            (Some(mut head1), Some(mut head2)) => {
                if head1.val < head2.val {
                    head1.next = Self::merge_two_lists(head1.next, Some(head2));
                    Some(head1)
                } else {
                    head2.next = Self::merge_two_lists(Some(head1), head2.next);
                    Some(head2)
                }
            }
        }
    }
}

#[test]
fn test() {
    let a = list!(1, 2, 4);
    let b = list!(1, 3, 4);
    let c = list!(1, 1, 2, 3, 4, 4);
    assert_eq!(Solution::merge_two_lists(a, b), c);
}
