struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n];
        let mut stk = vec![];
        let mut prev = 0;
        for log in logs {
            let mut iter = log.split(':');
            let id = iter.next().unwrap().parse::<usize>().unwrap();
            let action = match iter.next().unwrap() {
                "start" => Action::Begin,
                "end" => Action::End,
                _ => panic!(),
            };
            let stamp = iter.next().unwrap().parse::<i32>().unwrap();
            match action {
                Action::Begin => {
                    if let Some(&i) = stk.last() {
                        res[i] += stamp - prev;
                    }
                    prev = stamp;
                    stk.push(id);
                }
                Action::End => {
                    if let Some(i) = stk.pop() {
                        res[i] += stamp - prev + 1;
                        prev = stamp + 1;
                    }
                }
            }
        }
        res
    }
}

enum Action {
    Begin,
    End,
}

#[test]
fn test() {
    let n = 2;
    let logs = vec_string!["0:start:0", "1:start:2", "1:end:5", "0:end:6"];
    let res = vec![3, 4];
    assert_eq!(Solution::exclusive_time(n, logs), res);
}
