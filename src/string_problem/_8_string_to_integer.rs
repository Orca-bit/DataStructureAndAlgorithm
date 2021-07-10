struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim().chars().collect::<Vec<_>>();
        if s.is_empty() {
            return 0;
        }
        let mut pos = true;
        let mut res = 0;
        match s[0] {
            '+' => pos = true,
            '-' => pos = false,
            _ => {
                if !('0'..='9').contains(&s[0]) {
                    return 0;
                } else {
                    res = -((s[0] as u8 - b'0') as i32);
                }
            }
        }
        for &c in s[1..].iter() {
            if !('0'..='9').contains(&c) {
                break;
            }
            if let Some(val) = res.checked_mul(10) {
                res = val;
            } else {
                return Self::overflow(pos);
            }
            if let Some(val) = res.checked_add(-((c as u8 - b'0') as i32)) {
                res = val;
            } else {
                return Self::overflow(pos);
            }
        }
        if pos {
            if let Some(val) = res.checked_mul(-1) {
                res = val;
            } else {
                res = i32::MAX;
            }
        }
        res
    }

    fn overflow(pos: bool) -> i32 {
        if pos {
            i32::MAX
        } else {
            i32::MIN
        }
    }
}

#[test]
fn test() {
    let s = "42".to_string();
    let res = 42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "   -42".to_string();
    let res = -42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "4193 with words".to_string();
    let res = 4193;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "words and 987".to_string();
    let res = 0;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "-2147483648".to_string();
    let res = -2_147_483_648;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "2147483647".to_string();
    let res = 2_147_483_647;
    assert_eq!(Solution::my_atoi(s), res);
}
