
fn num_to_digit(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let v: Vec<char> = s.chars().collect();
    process(&v, 0)
}

fn process(v: &[char], i: usize) -> i32 {
    if i == v.len() {
        return 1;
    }
    if v[i] == '0' {
        return 0;
    }
    let mut res = process(v, i + 1);
    if i + 1 < v.len() && (v[i] as u8 - b'0') * 10 + (v[i + 1] as u8 - b'0') < 27 {
        res += process(v, i + 2);
    }
    res
}

#[test]
fn test() {
    let s = "01".to_string();
    assert_eq!(num_to_digit(s), 0);
}