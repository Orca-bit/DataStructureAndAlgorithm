use std::collections::BinaryHeap;
use std::cmp::min;
use std::cmp::Reverse;

struct SortArrDistLessK;

impl SortArrDistLessK {
    fn sort_arr_dist_less_k<T: Ord + Copy>(arr: &mut [T], k: usize) {
        let size = min(arr.len() - 1, k);
        let mut min_heap = BinaryHeap::with_capacity(size + 1);
        for i in 0..=size {
            min_heap.push(Reverse(arr[i]));
        }
        let mut i = 0;
        for index in (size + 1)..arr.len() {
            if let Some(Reverse(value)) = min_heap.pop() {
                arr[i] = value;
                min_heap.push(Reverse(arr[index]));
                i += 1;
            }
        }
        while !min_heap.is_empty() {
            if let Some(Reverse(value)) = min_heap.pop() {
                arr[i] = value;
                i += 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut v = vec![4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10];
    SortArrDistLessK::sort_arr_dist_less_k(&mut v, 3);
    println!("{:?}", v);
}