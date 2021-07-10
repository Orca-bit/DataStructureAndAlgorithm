use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    // 两个堆，存放小一半数据的堆为大根堆，存放大一半数据的堆为小根堆
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        // 先压入大一半的小根堆，取出大一半里最小的压入小一半的大根堆，
        // 并且确保两个堆的数据个数差不超过1，且小一半的个数更多
        self.hi.push(Reverse(num));
        let smallest = self.hi.pop().unwrap().0;
        self.lo.push(smallest);
        if self.lo.len() > self.hi.len() + 1 {
            self.hi.push(Reverse(self.lo.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        if (self.lo.len() + self.hi.len()) % 2 == 0 {
            (*self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.
        } else {
            *self.lo.peek().unwrap() as f64
        }
    }
}

#[test]
fn test() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    assert_eq!(obj.find_median(), 2.0);
}
