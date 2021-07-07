struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        // let mut res = 0;
        // let mut z = x ^ y;
        // while z != 0 {
        //     res += 1;
        //     z &= (z - 1);
        // }
        // res
        (x ^ y).count_ones() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
}