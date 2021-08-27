fn find_location(location: Vec<Vec<i32>>, map: Vec<Vec<char>>) -> i32 {
    let m = map.len();
    let n = map[0].len();
    let mut uf = UnionFind::new(m * n);
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '1' {
                continue;
            }
            if j > 0 && map[i][j - 1] == '0' {
                uf.union(m * i + j - 1, m * i + j);
            }
            if i > 0 && map[i - 1][j] == '0' {
                uf.union(m * (i - 1) + j, m * i + j);
            }
        }
    }

    if !uf.in_same_group(&location, m) {
        return -1;
    }
    let mut res = i32::MAX;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '0' {
                let mut tmp = i32::MIN;
                for v in &location {
                    let dist = (i as i32 - v[0]).abs() + (j as i32 - v[1]).abs();
                    tmp = tmp.max(dist);
                }
                res = res.min(tmp);
            }
        }
    }
    res
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

    fn in_same_group(&mut self, location: &Vec<Vec<i32>>, cols: usize) -> bool {
        let n = location.len();
        for i in 1..n {
            let a = cols * location[i - 1][0] as usize + location[i - 1][1] as usize;
            let b = cols * location[i][0] as usize + location[i][1] as usize;
            if self.find(a) != self.find(b) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let map = vec![
        vec!['0', '0', '0'],
        vec!['1', '0', '1'],
        vec!['0', '0', '0'],
    ];
    let location = vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]];
    assert_eq!(find_location(location, map), 2);
}
