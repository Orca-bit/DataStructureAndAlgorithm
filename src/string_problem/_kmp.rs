struct KMP;

impl KMP {
    pub fn get_index(s: &str, m: &str) -> isize {
        if s.is_empty() || m.is_empty() || m.len() < 1 || s.len() < m.len() {
            return -1;
        }
        let s: Vec<char> = s.chars().collect();
        let m: Vec<char> = m.chars().collect();
        let mut si = 0 as usize;
        let mut mi = 0 as usize;
        let next = Self::get_next_array(&m);
        while si < s.len() && mi < m.len() {
            if s[si] == m[mi] {                     // 第一种情况对应位置字符相等，一起向后走
                si += 1;
                mi += 1;
            } else {                                // 第二种情况，mi往前跳，直到开头或者满足情况一
                match next[mi] {
                    -1 => si += 1,
                    _ => mi = next[mi] as usize,
                }
            }
        }
        if mi == m.len() {
            (si - mi) as isize
        } else {
            -1
        }
    }

    fn get_next_array(s: &[char]) -> Vec<isize> {
        if s.len() == 1 {
            return vec![-1];
        }
        let mut next = vec![0; s.len()];
        next[0] = -1;
        let mut pos = 2;    // pos为当前处理的位置
        let mut cn = 0;     // cn为对比的位置
        while pos < next.len() {
            if s[pos - 1] == s[cn] {        // 第一种情况，当前位置与对比位置的字符相等
                cn += 1;
                next[pos] = cn as isize;
                pos += 1;
            } else if cn > 0 {              // 第二种情况，cn可以往前跳
                cn = next[cn] as usize;
            } else {                        // 第三种情况，cn到开头仍然没有匹配
                next[pos] = 0;
                pos += 1;
            }
        }
        next
    }
}


#[test]
fn test() {
    let s = "ababababac".to_string();
    let m = "abac".to_string();
    assert_eq!(KMP::get_index(&s, &m), 6);
}