struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let n = heights.len();
        if n == 0 {
            return res;
        }
        let m = heights[0].len();
        let mut visited = vec![vec![vec![false; m]; n]; 2];
        for i in 0..n {
            Self::dfs(i, 0, &heights, &mut visited[0]);
            Self::dfs(i, m - 1, &heights, &mut visited[1]);
        }
        for j in 0..m {
            Self::dfs(0, j, &heights, &mut visited[0]);
            Self::dfs(n - 1, j, &heights, &mut visited[1]);
        }
        for i in 0..n {
            for j in 0..m {
                if visited[0][i][j] && visited[1][i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }

    fn dfs(i: usize, j: usize, heights: &[Vec<i32>], visited: &mut Vec<Vec<bool>>) {
        if !visited[i][j] {
            visited[i][j] = true;
            if i > 0 && heights[i][j] <= heights[i - 1][j] {
                Self::dfs(i - 1, j, heights, visited);
            }
            if i < heights.len() - 1 && heights[i][j] <= heights[i + 1][j] {
                Self::dfs(i + 1, j, heights, visited);
            }
            if j > 0 && heights[i][j] <= heights[i][j - 1] {
                Self::dfs(i, j - 1, heights, visited);
            }
            if j < heights[0].len() - 1 && heights[i][j] <= heights[i][j + 1] {
                Self::dfs(i, j + 1, heights, visited);
            }
        }
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let res: Vec<Vec<i32>> = vec![
        vec![0, 4],
        vec![1, 3],
        vec![1, 4],
        vec![2, 2],
        vec![3, 0],
        vec![3, 1],
        vec![4, 0],
    ];
    assert_eq!(Solution::pacific_atlantic(matrix), res);
}
