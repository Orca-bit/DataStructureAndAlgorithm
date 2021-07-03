use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut root = Node::new();
        for word in word_dict.iter() {
            let mut node = &mut root;
            for ch in word.bytes() {
                let index = (ch - b'a') as usize;
                if node.children[index].is_none() {
                    node.children[index] = Some(Box::new(Node::new()));
                }
                node = node.children[index].as_mut().unwrap();
            }
            node.end = true;
        }
        let s = s.bytes().collect::<Vec<_>>();
        // dp[i] 表示s[i..]是否可以分割
        let mut dp = vec![false; s.len() + 1];
        dp[s.len()] = true;
        for i in (0..s.len()).rev() {
            let mut cur = &root;
            for j in i..s.len() {
                let path = (s[j] - b'a') as usize;
                if cur.children[path].is_none() {
                    break;
                }
                cur = cur.children[path].as_ref().unwrap();
                if cur.end && dp[j + 1] {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[0]
    }

    fn method_2(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let hs = word_dict.into_iter().collect::<HashSet<_>>();
        // dp[i] 表示s的前i个字符是否可以分割
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && hs.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

#[derive(Clone)]
struct Node {
    children: Vec<Option<Box<Node>>>,
    end: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: vec![None; 26],
            end: false,
        }
    }
}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Solution::word_break(s, word_dict), false);
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(Solution::method_2(s, word_dict), true);
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert_eq!(Solution::method_2(s, word_dict), true);
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Solution::method_2(s, word_dict), false);
}
