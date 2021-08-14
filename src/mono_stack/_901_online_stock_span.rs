#[derive(Default)]
struct StockSpanner {
    stk: Vec<(i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while let Some(top) = self.stk.pop() {
            if top.0 > price {
                self.stk.push(top);
                break;
            } else {
                res += top.1;
            }
        }
        self.stk.push((price, res));
        res as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

#[test]
fn test() {
    let mut obj = StockSpanner::new();
    assert_eq!(obj.next(100), 1);
    assert_eq!(obj.next(80), 1);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(70), 2);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(75), 4);
    assert_eq!(obj.next(85), 6);
}
