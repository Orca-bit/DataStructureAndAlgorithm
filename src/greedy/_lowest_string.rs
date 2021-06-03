// 返回字典序最小的拼接字符串
// 代码面试指南 p297
fn lowest_string(strs: &mut [&str]) -> String {
    let mut res = String::new();
    strs.sort_unstable_by(|&a, &b| {
        (a.to_owned() + b)
            .to_uppercase()
            .cmp(&(b.to_owned() + a).to_uppercase())
    });
    for &mut s in strs {
        res += s;
    }
    res.to_uppercase()
}

#[test]
fn test() {
    let mut strs = vec!["b", "ba"];
    let res = lowest_string(&mut strs);
    assert_eq!(res, "BAB".to_string());
}
