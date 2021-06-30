use super::util::*;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: ListLink, n: i32) -> ListLink {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut left = &mut dummy as *mut ListLink;
        let mut right = dummy.as_ref();
        for _ in 0..n {
            right = right?.next.as_ref();
        }
        while right?.next.is_some() {
            right = right?.next.as_ref();
            left = unsafe { &mut (*left).as_mut()?.next } as *mut _;
        }
        unsafe {
            (*left).as_mut()?.next = (*left).as_mut()?.next.as_mut()?.next.take();
        }
        dummy?.next
    }
}

#[test]
fn test() {
    let head = list![1, 2, 3, 4, 5];
    let res = list![1, 2, 3, 5];
    let n = 2;
    assert_eq!(Solution::remove_nth_from_end(head, n), res);
}
