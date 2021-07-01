use super::util::*;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<ListLink>) -> ListLink {
        if lists.is_empty() {
            return None;
        }
        if lists.len() == 1 {
            return lists.pop()?;
        }
        let mut deq: VecDeque<_> = lists.into_iter().collect();
        while deq.len() > 1 {
            let l1 = deq.pop_front()?;
            let l2 = deq.pop_front()?;
            deq.push_back(Self::merge(l1, l2));
        }
        deq.pop_front()?
    }

    fn merge(l1: ListLink, l2: ListLink) -> ListLink {
        match (l1, l2) {
            (None, None) => None,
            (Some(head1), None) => Some(head1),
            (None, Some(head2)) => Some(head2),
            (Some(mut head1), Some(mut head2)) => {
                if head1.val < head2.val {
                    head1.next = Self::merge(head1.next, Some(head2));
                    Some(head1)
                } else {
                    head2.next = Self::merge(Some(head1), head2.next);
                    Some(head2)
                }
            }
        }
    }
}

#[test]
fn test() {
    let lists = vec![list!(1, 4, 5), list!(1, 3, 4), list!(2, 6)];
    let res = list!(1, 1, 2, 3, 4, 4, 5, 6);
    assert_eq!(Solution::merge_k_lists(lists), res);
}
