// 程序员代码面试指南 p421
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_min_split_cost(arr: &[u32]) -> u32 {
    if arr.is_empty() || arr.len() < 2 {
        return 0;
    }
    let mut min_heap = BinaryHeap::with_capacity(arr.len());
    for &num in arr.iter() {
        min_heap.push(Reverse(num));
    }
    let mut res = 0;
    while let Some(Reverse(peek1)) = min_heap.pop() {
        if let Some(Reverse(peek2)) = min_heap.pop() {
            let sum = peek1 + peek2;
            res += sum;
            min_heap.push(Reverse(sum));
        }
    }
    res
}

#[test]
fn test() {
    let v = vec![10, 30, 20];
    assert_eq!(get_min_split_cost(&v), 90);
}
