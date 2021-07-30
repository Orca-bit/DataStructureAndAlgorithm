use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: HashMap<_, Vec<i32>> = HashMap::new();
        for pair in adjacent_pairs {
            let v = pair[0];
            let u = pair[1];
            adj.entry(v).or_default().push(u);
            adj.entry(u).or_default().push(v);
        }
        let n = adj.len();
        let mut res = vec![0; n];
        for (i, v) in adj.iter() {
            if v.len() == 1 {
                res[0] = *i;
                res[1] = v[0];
                break;
            }
        }
        for i in 2..n {
            let u = res[i - 1];
            res[i] = if adj[&u][0] == res[i - 2] {
                adj[&u][1]
            } else {
                adj[&u][0]
            };
        }
        res
    }
}

#[test]
fn test() {
    let adjacent_pairs = vec_vec_i32_1![[2, 1], [3, 4], [3, 2]];
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
    let adjacent_pairs = vec_vec_i32_1![[4, -2], [1, 4], [-3, 1]];
    let res = vec![-3, 1, 4, -2];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
    let adjacent_pairs = vec_vec_i32_1![[100000, -100000]];
    let res = vec![-100000, 100000];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
}
