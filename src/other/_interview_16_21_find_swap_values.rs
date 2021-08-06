use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_swap_values(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
        let sum1 = array1.iter().sum::<i32>();
        let sum2 = array2.iter().sum::<i32>();
        let diff = sum1 - sum2;
        if diff & 1 == 1 {
            return Vec::new();
        }
        let hs = array2.into_iter().collect::<HashSet<_>>();
        for x1 in array1 {
            let x2 = x1 - (diff >> 1);
            if hs.contains(&x2) {
                return vec![x1, x2];
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let array1 = vec![4, 1, 2, 1, 1, 2];
    let array2 = vec![3, 6, 3, 3];
    assert_eq!(Solution::find_swap_values(array1, array2), vec![4, 6]);
}
