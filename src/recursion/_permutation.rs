use std::collections::HashSet;

fn permutation(s: &str) -> Vec<String> {
    let mut v: Vec<char> = s.chars().collect();
    let mut res  = vec![];
    process(&mut v, 0, &mut res);
    res
}

fn process(v: &mut[char], i: usize, all: &mut Vec<String>) {
    if i == v.len() {
        let s = v.iter().collect::<String>();
        all.push(s);
        return;
    }
    let mut visited = HashSet::new();
    for j in i..v.len() {
        if !visited.contains(&v[j]) {
            visited.insert(v[j]);
            v.swap(i, j);
            process(v, i + 1, all);
            v.swap(i, j);
        }
    }
}

#[test]
fn test() {
    let s = "abca";
    let res = permutation(&s);
    println!("{:?}",res);
}