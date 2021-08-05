use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();
        let mut set = HashSet::new();
        for s in transactions {
            let mut v = s.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
            let city = v.pop().unwrap();
            let amount = v.pop().unwrap().parse::<i32>().unwrap();
            let time = v.pop().unwrap().parse::<i32>().unwrap();
            let name = v.pop().unwrap();
            map.entry(name).or_default().push((time, city, s.clone()));
            if amount > 1000 {
                set.insert(s.clone());
            }
        }
        for v in map.values() {
            let n = v.len();
            for i in 0..n {
                for j in i + 1..n {
                    let ti = &v[i];
                    let tj = &v[j];
                    if ti.1 != tj.1 && (ti.0 - tj.0).abs() <= 60 {
                        set.insert(ti.2.clone());
                        set.insert(tj.2.clone());
                    }
                }
            }
        }
        set.drain().collect()
    }
}

#[test]
fn test() {
    let transactions = vec![
        "alice,20,800,mtv".to_string(),
        "alice,50,100,beijing".to_string(),
    ];
    let mut res = vec![
        "alice,20,800,mtv".to_string(),
        "alice,50,100,beijing".to_string(),
    ];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let transactions = vec![
        "alice,20,800,mtv".to_string(),
        "alice,50,1200,mtv".to_string(),
    ];
    let mut res = vec!["alice,50,1200,mtv".to_string()];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let transactions = vec![
        "alice,20,800,mtv".to_string(),
        "bob,50,1200,mtv".to_string(),
    ];
    let mut res = vec!["bob,50,1200,mtv".to_string()];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
