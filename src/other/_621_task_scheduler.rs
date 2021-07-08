struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = vec![0; 26];
        let mut max_count = 0;
        for &task in tasks.iter() {
            map[(task as u8 - b'A') as usize] += 1;
            max_count = max_count.max(map[(task as u8 - b'A') as usize]);
        }
        let mut max_item = 0;
        for &count in map.iter() {
            if count == max_count {
                max_item += 1;
            }
        }
        (tasks.len() as i32).max((max_count - 1) * (n + 1) + max_item)
    }
}

#[test]
fn test() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    let res = 8;
    assert_eq!(Solution::least_interval(tasks, n), res);
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 0;
    let res = 6;
    assert_eq!(Solution::least_interval(tasks, n), res);
    let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let n = 2;
    let res = 16;
    assert_eq!(Solution::least_interval(tasks, n), res);
}