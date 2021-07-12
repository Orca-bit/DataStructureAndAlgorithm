struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut aba1 = 6_i64;
        let mut abc1 = 6_i64;
        for _ in 1..n {
            let aba2 = (aba1 * 3 + abc1 * 2) % MOD;
            let abc2 = (aba1 * 2 + abc1 * 2) % MOD;
            aba1 = aba2;
            abc1 = abc2;
        }
        ((aba1 + abc1) % MOD) as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 12;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 2;
    let res = 54;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 7;
    let res = 106494;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 5000;
    let res = 30228214;
    assert_eq!(Solution::num_of_ways(n), res);
}
