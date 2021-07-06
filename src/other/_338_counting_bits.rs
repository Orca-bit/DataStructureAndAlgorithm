struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n + 1];
        for i in 1..=n {
            res[i] = res[i >> 1] + (i & 1) as i32;
        }
        res
    }
}

#[test]
fn test() {
    let num = 5;
    let res = vec![0, 1, 1, 2, 1, 2];
    assert_eq!(Solution::count_bits(num), res);
}