struct Solution;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for v in dislikes {
            graph[v[0] as usize - 1].push(v[1] as usize - 1);
            graph[v[1] as usize - 1].push(v[0] as usize - 1);
        }
        let mut colors = vec![0; n];
        for i in 0..n {
            if colors[i] == 0 && !Self::dfs(i, 1, &mut colors, &graph) {
                return false;
            }
        }
        true
    }

    fn dfs(index: usize, color: i32, colors: &mut Vec<i32>, graph: &[Vec<usize>]) -> bool {
        colors[index] = color;
        for &hate in &graph[index] {
            if colors[hate] == color {
                return false;
            }
            if colors[hate] == 0 && !Self::dfs(hate, -color, colors, graph) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let n = 4;
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
    let res = true;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
    let n = 3;
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let res = false;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
    let n = 5;
    let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    let res = false;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
}
