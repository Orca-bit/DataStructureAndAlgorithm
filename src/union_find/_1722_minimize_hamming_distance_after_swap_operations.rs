use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut uf = UnionFind::new(n);
        for v in allowed_swaps {
            let (i, j) = (v[0] as usize, v[1] as usize);
            uf.union(i, j);
        }
        let mut source_groups: HashMap<_, HashMap<_, _>> = HashMap::new();
        let mut target_groups: HashMap<_, HashMap<_, _>> = HashMap::new();
        let mut groups: HashMap<_, HashSet<_>> = HashMap::new();
        for i in 0..n {
            let k = uf.find(i);
            *source_groups
                .entry(k)
                .or_default()
                .entry(source[i])
                .or_default() += 1;
            *target_groups
                .entry(k)
                .or_default()
                .entry(target[i])
                .or_default() += 1;
            groups.entry(k).or_default().insert(source[i]);
            groups.entry(k).or_default().insert(target[i]);
        }
        let mut pairs = 0;
        for (k, v) in &groups {
            for val in v {
                let source_cnt = source_groups[k].get(val).unwrap_or(&0);
                let target_cnt = target_groups[k].get(val).unwrap_or(&0);
                pairs += source_cnt.min(target_cnt);
            }
        }
        n as i32 - pairs
    }
}

struct UnionFind {
    size: usize,
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        Self { size, parent }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            j
        } else {
            self.parent[i] = self.find(j);
            self.parent[i]
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            let k = i.min(j);
            self.parent[i] = k;
            self.parent[j] = k;
        }
    }
}

#[test]
fn test() {
    let source = vec![1, 2, 3, 4];
    let target = vec![2, 1, 4, 5];
    let allowed_swaps = vec![vec![0, 1], vec![2, 3]];
    let res = 1;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
    let source = vec![1, 2, 3, 4];
    let target = vec![1, 3, 2, 4];
    let allowed_swaps = vec![];
    let res = 2;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
    let source = vec![5, 1, 2, 4, 3];
    let target = vec![1, 5, 4, 2, 3];
    let allowed_swaps = vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]];
    let res = 0;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
}
