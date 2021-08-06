struct Solution;

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let mut count = 0;
        let mut seed = 1;
        loop {
            if seed & 1 == 0 {
                seed >>= 1;
            } else {
                seed = (n + seed - 1) >> 1;
            }
            count += 1;
            if seed == 1 {
                break;
            }
        }
        count
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 1;
    assert_eq!(Solution::reinitialize_permutation(n), res);
    let n = 4;
    let res = 2;
    assert_eq!(Solution::reinitialize_permutation(n), res);
    let n = 6;
    let res = 4;
    assert_eq!(Solution::reinitialize_permutation(n), res);
}
