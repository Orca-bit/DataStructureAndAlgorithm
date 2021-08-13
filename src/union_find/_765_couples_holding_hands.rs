struct Solution;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut uf = UnionFind::new(n >> 1);
        let mut i = 0;
        while i < n {
            uf.union((row[i] >> 1) as usize, (row[i + 1] >> 1) as usize);
            i += 2;
        }
        ((n >> 1) - uf.get_groups()) as i32
    }
}

struct UnionFind {
    parent: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            groups: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.groups -= 1;
        }
    }

    fn get_groups(&self) -> usize {
        self.groups
    }
}

#[test]
fn test() {
    let row = vec![0, 2, 1, 3];
    let res = 1;
    assert_eq!(Solution::min_swaps_couples(row), res);
    let row = vec![3, 2, 0, 1];
    let res = 0;
    assert_eq!(Solution::min_swaps_couples(row), res);
}
