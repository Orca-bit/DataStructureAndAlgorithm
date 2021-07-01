struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        if n == 1 || n == 2 {
            return n;
        }
        let m = Self::matrix_power(vec![vec![1, 1], vec![1, 0]], n as u32 - 2);
        2 * m[0][0] + m[1][0]
    }

    fn matrix_power(m: Vec<Vec<i32>>, mut p: u32) -> Vec<Vec<i32>> {
        fn matrix_multiply(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut res = vec![vec![0; m2[0].len()]; m1.len()];
            for i in 0..m1.len() {
                for j in 0..m2[0].len() {
                    for k in 0..m2.len() {
                        res[i][j] += m1[i][k] * m2[k][j];
                    }
                }
            }
            res
        }
        let mut res = vec![vec![0; m[0].len()]; m.len()];
        for i in 0..res.len() {
            res[i][i] = 1;
        }
        let mut tmp = m;
        while p > 0 {
            if p & 1 != 0 {
                res = matrix_multiply(&res, &tmp);
            }
            tmp = matrix_multiply(&tmp, &tmp);
            p >>= 1;
        }
        res
    }

    fn other_method(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::other_method(3), 3);
    assert_eq!(Solution::other_method(2), 2);
    assert_eq!(Solution::other_method(1), 1);
}
