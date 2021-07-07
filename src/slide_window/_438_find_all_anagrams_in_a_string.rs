struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        if s.len() < p.len() {
            return res;
        }
        let s = s.bytes().collect::<Vec<_>>();
        let p = p.bytes().collect::<Vec<_>>();
        let mut s_map = vec![0; 26];
        let mut p_map = vec![0; 26];
        for &pc in p.iter() {
            p_map[(pc - b'a') as usize] += 1;
        }
        for &sc in s[0..p.len()].iter() {
            s_map[(sc - b'a') as usize] += 1;
        }
        let mut p1 = 0;
        let mut p2 = p.len() - 1;
        while p2 < s.len() {
            if s_map == p_map {
                res.push(p1 as i32);
            }
            s_map[(s[p1] - b'a') as usize] -= 1;
            p1 += 1;
            if p2 == s.len() - 1 {
                break;
            }
            p2 += 1;
            s_map[(s[p2] - b'a') as usize] += 1;
        }
        res
    }
}

#[test]
fn test() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);
    let s = "abab".to_string();
    let p = "ab".to_string();
    assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2]);
}
