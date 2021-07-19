struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut res = 0;
        while n != 0 {
            res += 1;
            n &= n - 1;
        }
        res
    }
}

#[test]
fn test() {
    let n = 0b00000000000000000000000000001011;
    let res = 3;
    assert_eq!(Solution::hammingWeight(n), res);
    let n = 0b00000000000000000000000010000000;
    let res = 1;
    assert_eq!(Solution::hammingWeight(n), res);
    let n = 0b11111111111111111111111111111101;
    let res = 31;
    assert_eq!(Solution::hammingWeight(n), res);
}
