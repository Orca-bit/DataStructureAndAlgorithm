use std::collections::HashMap;

#[derive(Default)]
struct FreqStack {
    freq: HashMap<i32, usize>,
    stack: HashMap<usize, Vec<i32>>,
    max: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        let n = self.freq.entry(val).or_default();
        *n += 1;
        self.max = self.max.max(*n);
        self.stack.entry(*n).or_default().push(val);
    }

    fn pop(&mut self) -> i32 {
        let max_freq = self.stack.get_mut(&self.max).unwrap();
        let val = max_freq.pop().unwrap();
        *self.freq.get_mut(&val).unwrap() -= 1;
        if max_freq.is_empty() {
            self.max -= 1;
        }
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

#[test]
fn test() {
    let mut obj = FreqStack::new();
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);
    assert_eq!(obj.pop(), 5);
    assert_eq!(obj.pop(), 7);
    assert_eq!(obj.pop(), 5);
    assert_eq!(obj.pop(), 4);
}
