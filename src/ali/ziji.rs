use std::cmp::{Ordering, max};
use std::io;

struct Pair {
    x: u32,
    y: u32,
}

impl Pair {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn process(mut pairs: Vec<Pair>) -> i32 {
    pairs.sort_unstable_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Equal => b.y.cmp(&a.y),
        other => other,
    });
    // 最长递增子序列
    let mut dp = vec![];
    for pair in pairs {
        if let Err(i) = dp.binary_search(&pair.y) {
            if i == dp.len() {
                dp.push(pair.y);
            } else {
                dp[i] = pair.y;
            }
        }
    }
    dp.len() as i32
}

#[test]
fn test() {
    let v = vec![Pair::new(1, 0), Pair::new(1, 10), Pair::new(3, 2), Pair::new(2, 3)];
    assert_eq!(process(v), 2);
}

fn main1() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let _ = stdin.read_line(&mut buf);
    let n = buf.trim().parse().unwrap();
    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        let _ = stdin.read_line(&mut buf);
        let m: usize = buf.trim().parse().unwrap();
        buf.clear();
        for _ in 0..2 {
            let _ = stdin.read_line(&mut buf);
            buf.push(' ');
        }
        let v: Vec<_> = buf.split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect();
        let mut pairs = Vec::with_capacity(m);
        for i in 0..m {
            pairs.push(Pair::new(v[i], v[i + m]));
        }
        data.push(pairs);
    }
    for pairs in data {
        println!("{}", process(pairs));
    }
}
