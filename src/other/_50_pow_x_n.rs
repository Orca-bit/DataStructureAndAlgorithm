struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.;
        }
        if n == i32::MIN {
            return if x == 1. || x == -1. { 1. } else { 0. };
        }
        let mut res = 1.;
        let mut pow = n.abs();
        let mut tmp = x;
        while pow != 0 {
            if pow & 1 != 0 {
                res *= tmp;
            }
            tmp *= tmp;
            pow >>= 1;
        }
        if n > 0 {
            res
        } else {
            1. / res
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    assert_eq!(Solution::my_pow(2.0, -3), 0.125);
    assert_eq!(Solution::my_pow(1.0, -2_147_483_648), 1.0);
}