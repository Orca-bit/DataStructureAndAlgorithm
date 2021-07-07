use std::cmp::Ordering::{Equal, Greater, Less};

struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            Equal => a[1].cmp(&b[1]),
            Greater => Less,
            Less => Greater,
        });
        let mut res = vec![];
        for p in people.iter() {
            res.insert(p[1] as usize, p.clone());
        }
        res
    }
}

#[test]
fn test() {
    let people = vec![
        vec![7, 0],
        vec![4, 4],
        vec![7, 1],
        vec![5, 0],
        vec![6, 1],
        vec![5, 2],
    ];
    let res = vec![
        vec![5, 0],
        vec![7, 0],
        vec![5, 2],
        vec![6, 1],
        vec![4, 4],
        vec![7, 1],
    ];
    assert_eq!(Solution::reconstruct_queue(people), res);
}
