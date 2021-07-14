struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect::<Vec<_>>();
        let mut pairs = vec![];
        let mut res = vec![];
        Self::process(&mut res, &mut pairs, 0, &s);
        res
    }

    fn process(
        res: &mut Vec<Vec<String>>,
        pairs: &mut Vec<(usize, usize)>,
        index: usize,
        s: &Vec<char>,
    ) {
        if index == s.len() {
            let mut path = vec![];
            for &(si, ei) in pairs.iter() {
                path.push(s[si..ei].iter().collect::<String>());
            }
            res.push(path);
            return;
        }
        for end in index + 1..=s.len() {
            if Self::is_palindrome(&s[index..end]) {
                pairs.push((index, end));
                Self::process(res, pairs, end, s);
                pairs.pop();
            }
        }
    }

    fn is_palindrome(s: &[char]) -> bool {
        if s.len() < 2 {
            return true;
        }
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    let mut res = vec![
        vec!["aa".to_string(), "b".to_string()],
        vec!["a".to_string(), "a".to_string(), "b".to_string()],
    ];
    let mut ans = Solution::partition(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = "efe".to_string();
    let mut res = vec![
        vec!["e".to_string(), "f".to_string(), "e".to_string()],
        vec!["efe".to_string()],
    ];
    let mut ans = Solution::partition(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
