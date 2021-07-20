struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut num_s = vec![];
        let mut op_s = vec![];
        let s = s.as_bytes();
        let mut i = 0;
        while i < s.len() {
            match s[i] {
                b' ' => i += 1,
                b'*' => {
                    i += 1;
                    *num_s.last_mut().unwrap() *= Self::parse_num(s, &mut i);
                }
                b'/' => {
                    i += 1;
                    *num_s.last_mut().unwrap() /= Self::parse_num(s, &mut i);
                }
                b'+' | b'-' => {
                    op_s.push(s[i]);
                    i += 1;
                }
                _ => num_s.push(Self::parse_num(s, &mut i)),
            }
        }
        let mut res = num_s[0];
        num_s
            .into_iter()
            .skip(1)
            .zip(op_s.into_iter())
            .for_each(|(num, op)| {
                if op == b'+' {
                    res += num;
                } else {
                    res -= num;
                }
            });
        res
    }

    fn parse_num(s: &[u8], i: &mut usize) -> i32 {
        let mut res = 0;
        while *i < s.len() && s[*i] == b' ' {
            *i += 1;
        }
        while *i < s.len() && s[*i].is_ascii_digit() {
            res = res * 10 + (s[*i] - b'0') as i32;
            *i += 1;
        }
        res
    }
}

#[test]
fn test() {
    // let s = "3+2*2".to_string();
    // assert_eq!(Solution::calculate(s), 7);
    // let s = "0-0".to_string();
    // assert_eq!(Solution::calculate(s), 0);
    let s = "42".to_string();
    assert_eq!(Solution::calculate(s), 42);
    // let s = "0-2147483647".to_string();
    // assert_eq!(Solution::calculate(s), -2_147_483_647);
}
