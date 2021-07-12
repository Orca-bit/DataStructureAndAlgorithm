struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let last = Self::count_and_say(n - 1).chars().collect::<Vec<_>>();
        let mut res = String::new();
        let mut times = 1;
        for i in 1..last.len() {
            if last[i] == last[i - 1] {
                times += 1;
            } else {
                res.push_str(times.to_string().as_str());
                res.push_str(last[i - 1].to_string().as_str());
                times = 1;
            }
        }
        res.push_str(times.to_string().as_str());
        res.push_str(last[last.len() - 1].to_string().as_str());
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_and_say(4), String::from("1211"));
    assert_eq!(Solution::count_and_say(1), String::from("1"));
}