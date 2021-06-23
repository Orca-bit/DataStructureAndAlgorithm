struct Solution;

use std::cmp::max;
use std::cmp::Ordering::*;
use std::collections::VecDeque;

impl Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.is_empty() {
            return vec![];
        }
        let mut deque: VecDeque<Vec<Vec<i32>>> = buildings
            .iter()
            .map(|x| vec![vec![x[0], x[2]], vec![x[1], 0]])
            .collect();
        while deque.len() > 1 {
            let a = deque.pop_front().unwrap();
            let b = deque.pop_front().unwrap();
            let c = Self::merge(a, b);
            deque.push_back(c);
        }
        deque.pop_front().unwrap()
    }

    fn merge(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut l = 0;
        let mut r = 0;
        let mut x = 0;
        let mut pre_h = 0;
        let mut res = vec![];
        while i < a.len() && j < b.len() {
            match a[i][0].cmp(&b[j][0]) {
                Equal => {
                    x = a[i][0];
                    l = a[i][1];
                    r = b[j][1];
                    i += 1;
                    j += 1;
                }
                Less => {
                    x = a[i][0];
                    l = a[i][1];
                    i += 1;
                }
                Greater => {
                    x = b[j][0];
                    r = b[j][1];
                    j += 1;
                }
            }
            let h = max(l, r);
            if h != pre_h {
                res.push(vec![x, h]);
                pre_h = h;
            }
        }
        while i < a.len() {
            x = a[i][0];
            let h = a[i][1];
            i += 1;
            if h != pre_h {
                res.push(vec![x, h]);
                pre_h = h;
            }
        }
        while j < b.len() {
            x = b[j][0];
            let h = b[j][1];
            j += 1;
            if h != pre_h {
                res.push(vec![x, h]);
                pre_h = h;
            }
        }
        res
    }
}

#[test]
fn test() {
    let buildings = vec_vec_i32_1![
        [2, 9, 10],
        [3, 7, 15],
        [5, 12, 12],
        [15, 20, 10],
        [19, 24, 8]
    ];
    let res = vec![
        [2, 10],
        [3, 15],
        [7, 12],
        [12, 0],
        [15, 10],
        [20, 8],
        [24, 0],
    ];
    assert_eq!(Solution::get_skyline(buildings), res);
}
