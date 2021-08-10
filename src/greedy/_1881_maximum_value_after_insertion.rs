struct Solution;

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut n = n.as_bytes().to_owned();
        let size = n.len();
        let neg = if n[0] == b'-' { true } else { false };
        if neg {
            for i in 1..size {
                if (n[i] - b'0') as i32 > x {
                    n.insert(i, b'0' + x as u8);
                    break;
                } else if i == size - 1 {
                    n.push(b'0' + x as u8);
                } else {
                    continue;
                }
            }
        } else {
            for i in 0..size {
                if ((n[i] - b'0') as i32) < x {
                    n.insert(i, b'0' + x as u8);
                    break;
                } else if i == size - 1 {
                    n.push(b'0' + x as u8);
                } else {
                    continue;
                }
            }
        }
        n.iter().map(|&c| c as char).collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_value("-13".to_string(), 2),
        "-123".to_string()
    );
    assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
}
