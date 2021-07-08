struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut index_map = vec![0; 26];
        let s = s.bytes().collect::<Vec<_>>();
        for (i, &c) in s.iter().enumerate() {
            index_map[(c - b'a') as usize] = i;
        }
        let mut left = 0;
        let mut right = index_map[(s[0] - b'a') as usize];
        let mut res = vec![];
        for i in 1..s.len() {
            if i > right {
                res.push((right - left + 1) as i32);
                left = i;
            }
            right = right.max(index_map[(s[i] - b'a') as usize]);
        }
        res.push((right - left + 1) as i32);
        res
    }
}

#[test]
fn test() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let res = vec![9, 7, 8];
    assert_eq!(Solution::partition_labels(s), res);
    let s = "vhaagbqkaq".to_string();
    let res = vec![1, 1, 8];
    assert_eq!(Solution::partition_labels(s), res);
}
