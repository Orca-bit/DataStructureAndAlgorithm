struct Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 0 {
            n /= 5;
            res += n;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(10), 2);
}
