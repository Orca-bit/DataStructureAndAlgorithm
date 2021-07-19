struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut res = 0;
        let mut i = 0;
        while i < 32 && x > 0 {
            res |= (x & 1) << (31 - i);
            i += 1;
            x >>= 1;
        }
        res
    }
}

#[test]
fn test() {
    let n = 0b00000010100101000001111010011100;
    let res = 964176192;
    assert_eq!(Solution::reverse_bits(n), res);
    let n = 0b11111111111111111111111111111101;
    let res = 3221225471;
    assert_eq!(Solution::reverse_bits(n), res);
}