#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

pub type ListLink = Option<Box<ListNode>>;

pub trait ListMaker {
    fn link(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for ListLink {}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e:expr) => {
        ListLink::link($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}
