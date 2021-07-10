struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let pos = x.is_positive();
        if pos {
            x = -x;
        }
        let mut res = 0;
        let m = i32::MIN / 10;
        let o = i32::MIN % 10;
        while x != 0 {
            if res < m || (res == m && x % 10 < o) {
                return 0;
            }
            res = res * 10 + x % 10;
            x /= 10;
        }
        if pos {
            -res
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let x = 2_147_483_647;
    let res = 0;
    assert_eq!(Solution::reverse(x), res);
    let x = 123_456_789;
    let res = 987_654_321;
    assert_eq!(Solution::reverse(x), res);
    let x = -123;
    let res = -321;
    assert_eq!(Solution::reverse(x), res);
}
