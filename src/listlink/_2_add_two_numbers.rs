use super::util::*;

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        let mut res = None;
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut p3 = &mut res;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;
            if let Some(node1) = p1.as_ref() {
                val += node1.val;
                p1 = &node1.next;
            }
            if let Some(node2) = p2.as_ref() {
                val += node2.val;
                p2 = &node2.next;
            }
            carry = val / 10;
            *p3 = Some(Box::new(ListNode::new(val % 10)));
            p3 = &mut p3.as_mut().unwrap().next;
        }
        res
    }
}

#[test]
fn test() {
    let l1 = list!(2, 4, 3);
    let l2 = list!(5, 6, 4);
    let l3 = list!(7, 0, 8);
    assert_eq!(Solution::add_two_numbers(l1, l2), l3);
}