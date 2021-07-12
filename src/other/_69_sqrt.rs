struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x < 4 {
            return 1;
        }
        let mut l = 1_i64;
        let mut r = x as i64;
        let mut res = 1;
        while l <= r {
            let mid = (l + r) / 2;
            if mid * mid <= x as i64 {
                res = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        res as i32
    }

    fn newton_iter(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut res = x as f64;
        let c = x as f64;
        loop {
            let tmp = (res + c / res) * 0.5;
            if (tmp - res).abs() < 0.0000001 {
                return tmp as i32;
            }
            res = tmp;
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(5), 2);
    assert_eq!(Solution::my_sqrt(2147395599), 46339);
    assert_eq!(Solution::newton_iter(8), 2);
    assert_eq!(Solution::newton_iter(5), 2);
    assert_eq!(Solution::newton_iter(2147395599), 46339);
}