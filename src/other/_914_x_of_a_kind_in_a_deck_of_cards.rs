use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut res = 0;
        let mut hm = HashMap::new();
        for x in deck {
            *hm.entry(x).or_default() += 1;
        }
        for &y in hm.values() {
            res = Self::gcd(res, y);
        }
        res >= 2
    }

    fn gcd(x: i32, y: i32) -> i32 {
        if x == 0 {
            y
        } else {
            Self::gcd(y % x, x)
        }
    }
}

#[test]
fn test() {
    let deck = vec![1, 2, 3, 4, 4, 3, 2, 1];
    assert_eq!(Solution::has_groups_size_x(deck), true);
    let deck = vec![1, 1, 1, 2, 2, 2, 3, 3];
    assert_eq!(Solution::has_groups_size_x(deck), false);
    let deck = vec![1];
    assert_eq!(Solution::has_groups_size_x(deck), false);
    let deck = vec![1, 1];
    assert_eq!(Solution::has_groups_size_x(deck), true);
    let deck = vec![1, 1, 2, 2, 2, 2];
    assert_eq!(Solution::has_groups_size_x(deck), true);
}
