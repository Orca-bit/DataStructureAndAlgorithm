use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn build_map(map: &mut HashMap<usize, Vec<usize>>) {
    for line in io::stdin().lock().lines() {
        if let Ok(s) = line {
            let v: Vec<_> = s
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let value = map.entry(v[0]).or_default();
            value.push(v[1]);
        }
    }
}

struct ReturnData {
    have: i32,
    no: i32,
}

impl ReturnData {
    fn new(have: i32, no: i32) -> Self {
        Self { have, no }
    }
}

fn process(map: &mut HashMap<usize, Vec<usize>>, id: usize, happy: &[i32]) -> ReturnData {
    let mut have_this = happy[id - 1];
    let mut no_this = 0;
    if !map.contains_key(&id) {
        return ReturnData::new(have_this, no_this);
    }
    for i in map.get(&id).unwrap().clone() {
        let i_data = process(map, i, happy);
        have_this += i_data.no;
        no_this += max(i_data.no, i_data.have);
    }
    ReturnData::new(have_this, no_this)
}

#[test]
fn test() {
    let mut s1 = String::new();
    let _ = io::stdin().read_line(&mut s1);
    let v: Vec<_> = s1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    s1.clear();
    let _ = io::stdin().read_line(&mut s1);
    let happy: Vec<_> = s1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut map = HashMap::with_capacity(v[0]);
    build_map(&mut map);
    let leader_id = v[1];
    let res = process(&mut map, leader_id, &happy);
    println!("{}", max(res.have, res.no));
}
