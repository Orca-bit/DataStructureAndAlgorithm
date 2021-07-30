use super::util::*;
use rand::prelude::*;

struct Solution {
    head: ListLink,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let rng = thread_rng();
        Self { head, rng }
    }

    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let mut cur = &self.head;
        let mut res = 0;
        let mut i = 0;
        while let Some(node) = cur {
            i += 1;
            if self.rng.gen_range(0..i) == 0 {
                res = node.val;
            }
            cur = &node.next;
        }
        res
    }
}
