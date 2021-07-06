use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    num: i32,
    times: usize,
}

impl Pair {
    fn new(num: i32, times: usize) -> Self {
        Self { num, times }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.times.cmp(&self.times)
    }
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k < 1 {
            return Vec::new();
        }
        let k = k as usize;
        let mut map = HashMap::new();
        for &num in nums.iter() {
            *map.entry(num).or_default() += 1;
        }
        let mut heap = BinaryHeap::with_capacity(k);
        for (num, times) in map {
            let pair = Pair::new(num, times);
            if heap.len() < k {
                heap.push(pair);
            } else {
                if pair < *heap.peek().unwrap() {
                    heap.pop();
                    heap.push(pair);
                }
            }
        }
        heap.iter().map(|a| a.num).collect()
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = vec![1, 2];
    assert_eq!(Solution::top_k_frequent(nums, k), res);
}