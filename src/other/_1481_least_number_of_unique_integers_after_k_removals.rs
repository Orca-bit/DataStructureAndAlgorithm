use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut map: HashMap<_, i32> = HashMap::new();
        for x in arr {
            *map.entry(x).or_default() += 1;
        }
        let mut q = BinaryHeap::new();
        for (_, &v) in &map {
            q.push(Reverse(v));
        }
        while let Some(v) = q.peek() {
            if v.0 <= k {
                k -= v.0;
                q.pop();
            } else {
                break;
            }
        }
        q.len() as i32
    }
}

#[test]
fn test() {
    let arr = vec![5, 5, 4];
    let k = 1;
    let res = 1;
    assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), res);
    let arr = vec![4, 3, 1, 1, 3, 3, 2];
    let k = 3;
    let res = 2;
    assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), res);
}
