struct Solution;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        mut num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut q = values.into_iter().zip(labels).collect::<BinaryHeap<_>>();
        let mut map: HashMap<_, i32> = HashMap::new();
        let mut res = 0;
        while let Some(v) = q.pop() {
            let freq = map.entry(v.1).or_default();
            if *freq < use_limit {
                res += v.0;
                *freq += 1;
                num_wanted -= 1;
            }
            if num_wanted == 0 {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 1, 2, 2, 3];
    let num_wanted = 3;
    let use_limit = 1;
    let res = 9;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 3, 3, 3, 2];
    let num_wanted = 3;
    let use_limit = 2;
    let res = 12;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![9, 8, 8, 7, 6];
    let labels = vec![0, 0, 0, 1, 1];
    let num_wanted = 3;
    let use_limit = 1;
    let res = 16;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![9, 8, 8, 7, 6];
    let labels = vec![0, 0, 0, 1, 1];
    let num_wanted = 3;
    let use_limit = 2;
    let res = 24;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
}
