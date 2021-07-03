#[derive(Default)]
struct MinStack {
    nums: Vec<i32>,
    min_s: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        self.nums.push(val);
        match self.min_s.last() {
            Some(&old) => self.min_s.push(old.min(val)),
            None => self.min_s.push(val),
        }
    }

    fn pop(&mut self) {
        if !self.nums.is_empty() && !self.min_s.is_empty() {
            self.nums.pop();
            self.min_s.pop();
        }
    }

    fn top(&self) -> i32 {
        match self.nums.last() {
            Some(&val) => val,
            None => panic!("Stack is empty!"),
        }
    }

    fn get_min(&self) -> i32 {
        match self.min_s.last() {
            Some(&min) => min,
            None => panic!("Stack is empty!"),
        }
    }
}

#[test]
fn test() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}