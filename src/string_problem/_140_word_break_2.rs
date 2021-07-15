struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        trie.construct(word_dict);
        let s = s.chars().collect::<Vec<_>>();
        let dp = Self::dp(&s, &trie);
        let mut res = vec![];
        let mut path = vec![];
        Self::process(&mut res, &mut path, &s, 0, &trie, &dp);
        res
    }

    fn process(
        res: &mut Vec<String>,
        path: &mut Vec<String>,
        s: &[char],
        index: usize,
        trie: &Trie,
        dp: &[bool],
    ) {
        if index == s.len() {
            if !path.is_empty() {
                let mut cur = String::new();
                cur.push_str(path[0].as_str());
                for s in path.iter().skip(1) {
                    cur.push(' ');
                    cur.push_str(s.as_str());
                }
                res.push(cur);
            }
            return;
        }
        let mut cur = trie;
        for i in index..s.len() {
            let index = (s[i] as u8 - b'a') as usize;
            if cur.children[index].is_none() {
                break;
            }
            cur = cur.children[index].as_ref().unwrap();
            if let Some(ref ts) = cur.end {
                if dp[i + 1] {
                    path.push(ts.clone());
                    Self::process(res, path, s, i + 1, trie, dp);
                    path.pop();
                }
            }
        }
    }

    fn dp(s: &[char], trie: &Trie) -> Vec<bool> {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[n] = true;
        for i in (0..n).rev() {
            let mut cur = trie;
            for end in i..n {
                let index = (s[end] as u8 - b'a') as usize;
                if cur.children[index].is_none() {
                    break;
                }
                cur = cur.children[index].as_ref().unwrap();
                if let Some(ref s) = cur.end {
                    if dp[end + 1] {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }
        dp
    }
}

#[derive(Clone)]
struct Trie {
    end: Option<String>,
    children: Vec<Option<Box<Trie>>>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            end: None,
            children: vec![None; 26],
        }
    }

    pub fn construct(&mut self, words: Vec<String>) {
        for word in words.iter() {
            let mut cur = &mut *self;
            let w = word.chars().collect::<Vec<_>>();
            for c in w.into_iter() {
                let index = c as u8 - b'a';
                if cur.children[index as usize].is_none() {
                    cur.children[index as usize] = Some(Box::new(Trie::new()));
                }
                cur = cur.children[index as usize].as_mut().unwrap();
            }
            cur.end = Some(word.clone());
        }
    }
}

#[test]
fn test() {
    let s = "catsanddog".to_string();
    let word_dict = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string(),
    ];
    let mut res = vec!["cats and dog".to_string(), "cat sand dog".to_string()];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "pineapplepenapple".to_string();
    let word_dict = vec![
        "apple".to_string(),
        "pen".to_string(),
        "applepen".to_string(),
        "pine".to_string(),
        "pineapple".to_string(),
    ];
    let mut res = vec![
        "pine apple pen apple".to_string(),
        "pineapple pen apple".to_string(),
        "pine applepen apple".to_string(),
    ];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    let mut res: Vec<String> = vec![];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
