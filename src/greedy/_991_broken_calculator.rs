struct Solution;

impl Solution {
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut ops = 0;
        while start_value < target {
            if target & 1 == 0 {
                target >>= 1;
            } else {
                target += 1;
            }
            ops += 1;
        }
        start_value - target + ops
    }
}

#[test]
fn test() {
    let x = 2;
    let y = 3;
    let res = 2;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 5;
    let y = 8;
    let res = 2;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 3;
    let y = 10;
    let res = 3;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 1024;
    let y = 1;
    let res = 1023;
    assert_eq!(Solution::broken_calc(x, y), res);
}
