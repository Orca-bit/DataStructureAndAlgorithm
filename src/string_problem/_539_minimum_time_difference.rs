use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let n = time_points.len();
        if n > 1440 {
            return 0;
        }
        let mut v = BTreeSet::new();
        for x in time_points {
            let num = Self::to_num(x);
            if v.contains(&num) {
                return 0;
            }
            v.insert(num);
        }
        let mut first = None;
        let mut last = None;
        let mut prev = None;
        let mut res = usize::MAX;
        for x in v {
            if first.is_none() {
                first = Some(x);
                prev = Some(x);
            } else {
                res = res.min(x - prev.unwrap());
                last = Some(x);
                prev = Some(x);
            }
        }
        res.min(first.unwrap() + 1440 - last.unwrap()) as i32
    }

    fn to_num(s: String) -> usize {
        let v = s
            .split(':')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        v[0] * 60 + v[1]
    }
}

#[test]
fn test() {
    let time_points = vec!["23:59".to_string(), "00:00".to_string()];
    assert_eq!(Solution::find_min_difference(time_points), 1);
}
