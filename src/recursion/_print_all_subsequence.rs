fn print_all_subsequence(s: &str) {
    let mut v: Vec<char> = s.chars().collect();
    process(&mut v, 0);
}

fn process(v: &mut Vec<char>, i: usize) {
    if i == v.len() {
        let s = v
            .iter()
            .filter(|&&c| !c.is_whitespace())
            .collect::<String>();
        println!("{}", s);
        return;
    }
    process(v, i + 1);
    let temp = v[i];
    v[i] = ' ';
    process(v, i + 1);
    v[i] = temp;
}

#[test]
fn test() {
    print_all_subsequence("abc");
}
