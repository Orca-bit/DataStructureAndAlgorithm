fn remove(s: String) -> String {
    let mut s = s.as_bytes().iter().map(|&c| Some(c)).collect::<Vec<_>>();
    let mut n = s.len();
    let mut map = vec![0; 256];
    for i in 0..n {
        if map[s[i].unwrap() as usize] != 0 {
            s[i] = None;
            continue;
        }
        map[s[i].unwrap() as usize] = 1;
    }
    String::from_utf8(
        s.iter()
            .filter(|c| c.is_some())
            .map(|c| (*c).unwrap())
            .collect(),
    )
    .unwrap()
}

#[test]
fn test() {
    let s = "hello".to_string();
    let res = "helo".to_string();
    assert_eq!(remove(s), res);
    let s = "aaaaa".to_string();
    let res = "a".to_string();
    assert_eq!(remove(s), res);
}
