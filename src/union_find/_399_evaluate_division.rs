use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let n = equations.len();
        let mut union_find = UnionFind::new(n * 2);
        let mut map = HashMap::with_capacity(n * 2);
        let mut id = 0;
        for (i, eq) in equations.iter().enumerate() {
            let var1 = eq[0].clone();
            let var2 = eq[1].clone();
            if !map.contains_key(&var1) {
                map.insert(var1.clone(), id);
                id += 1;
            }
            if !map.contains_key(&var2) {
                map.insert(var2.clone(), id);
                id += 1;
            }
            union_find.union(
                *map.get(&var1).unwrap(),
                *map.get(&var2).unwrap(),
                values[i],
            );
        }

        let mut res = vec![0.; queries.len()];
        for (i, que) in queries.iter().enumerate() {
            let var1 = que[0].clone();
            let var2 = que[1].clone();
            let id1 = map.get(&var1);
            let id2 = map.get(&var2);
            if id1.is_none() || id2.is_none() {
                res[i] = -1.;
            } else {
                res[i] = union_find.connect(*id1.unwrap(), *id2.unwrap());
            }
        }
        res
    }
}

struct UnionFind {
    parent: Vec<usize>,
    weights: Vec<f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            weights: vec![1.; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            self.weights[i] *= self.weights[j];
            k
        }
    }

    fn union(&mut self, i: usize, j: usize, value: f64) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            return;
        }
        self.parent[root_i] = root_j;
        self.weights[root_i] = value * self.weights[j] / self.weights[i];
    }

    fn connect(&mut self, i: usize, j: usize) -> f64 {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            self.weights[i] / self.weights[j]
        } else {
            -1.
        }
    }
}

#[test]
fn test() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    let res = vec![6.0, 0.5, -1.0, 1.0, -1.0];
    assert_eq!(Solution::calc_equation(equations, values, queries), res);
}
