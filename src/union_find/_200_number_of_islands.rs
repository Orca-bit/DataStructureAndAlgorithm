struct Solution;

impl Solution {
    pub fn num_islands1(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut res = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        if rows == 0 || cols == 0 {
            return res;
        }
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    res += 1;
                    Self::infect(
                        &mut grid,
                        i as isize,
                        j as isize,
                        rows as isize,
                        cols as isize,
                    );
                }
            }
        }
        res
    }

    // 并查集解法
    pub fn num_islands2(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();
        let mut uf = UnionFind::new(rows * cols + 1); // 多给一个元素记录‘0’
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    if j > 0 && grid[i][j - 1] == '1' {
                        uf.union(i * cols + j, i * cols + j - 1);
                    }
                    if i > 0 && grid[i - 1][j] == '1' {
                        uf.union(i * cols + j, (i - 1) * cols + j);
                    }
                } else {
                    uf.union(i * cols + j, rows * cols);
                }
            }
        }
        uf.get_groups() as i32 - 1
    }

    fn infect(grid: &mut Vec<Vec<char>>, row: isize, col: isize, rows: isize, cols: isize) {
        if row < 0
            || row >= rows
            || col < 0
            || col >= cols
            || grid[row as usize][col as usize] != '1'
        {
            return;
        }
        grid[row as usize][col as usize] = '2';
        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (i, j) in neighbors {
            Self::infect(grid, row + i, col + j, rows, cols);
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            groups: n,
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if j == i {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    pub fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.groups -= 1;
        }
    }

    pub fn get_groups(&self) -> usize {
        self.groups
    }
}

#[macro_export]
macro_rules! vec_vec_char {
    ($($tail:tt),*) => {
        vec![$(vec!$tail),*] as Vec<Vec<char>>
    };
}

#[test]
fn test() {
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0']
    ];
    assert_eq!(Solution::num_islands1(grid), 1);
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '0', '0', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '1', '0', '0'],
        ['0', '0', '0', '1', '1']
    ];
    assert_eq!(Solution::num_islands2(grid), 3);
}
