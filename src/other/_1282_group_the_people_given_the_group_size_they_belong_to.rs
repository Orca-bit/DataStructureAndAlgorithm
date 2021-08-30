use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hm: HashMap<_, Vec<_>> = HashMap::new();
        for (i, &size) in group_sizes.iter().enumerate() {
            hm.entry(size).or_default().push(i);
        }
        let mut res = vec![];
        for (k, v) in hm {
            let mut cache = vec![];
            for i in v {
                cache.push(i as i32);
                if cache.len() == k as usize {
                    res.push(cache.clone());
                    cache.clear();
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
    );
}
