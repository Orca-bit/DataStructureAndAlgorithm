struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        Self::process(&s, 0).res
    }

    fn process(s: &[char], mut index: usize) -> Data {
        let mut str = String::new();
        let mut times = 0;
        while index < s.len() && s[index] != ']' {
            if ('a'..='z').contains(&s[index]) || ('A'..='Z').contains(&s[index]) {
                str.push(s[index]);
            } else if ('0'..='9').contains(&s[index]) {
                times = times * 10 + (s[index] as u8 - b'0') as i32;
            } else {
                let next = Self::process(s, index + 1);
                index = next.end;
                for _ in 0..times {
                    str.push_str(next.res.as_str());
                }
                times = 0;
            }
            index += 1;
        }
        Data::new(str, index)
    }
}

struct Data {
    res: String,
    end: usize,
}

impl Data {
    fn new(res: String, end: usize) -> Self {
        Self { res, end }
    }
}

#[test]
fn test() {
    let s = "3[a]2[bc]".to_string();
    let res = "aaabcbc".to_string();
    assert_eq!(Solution::decode_string(s), res);
    let s = "3[a2[c]]".to_string();
    let res = "accaccacc".to_string();
    assert_eq!(Solution::decode_string(s), res);
    let s = "2[abc]3[cd]ef".to_string();
    let res = "abcabccdcdcdef".to_string();
    assert_eq!(Solution::decode_string(s), res);
    let s = "abc3[cd]xyz".to_string();
    let res = "abccdcdcdxyz".to_string();
    assert_eq!(Solution::decode_string(s), res);
}
