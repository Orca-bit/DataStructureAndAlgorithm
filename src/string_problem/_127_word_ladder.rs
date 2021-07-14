use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let s = begin_word.as_bytes().to_vec();
        let e = end_word.as_bytes().to_vec();
        let word_list = word_list
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<HashSet<_>>();
        if !word_list.contains(&e) {
            return 0;
        }
        Self::bfs(s, e, word_list) + 1
    }

    fn bfs(s: Vec<u8>, e: Vec<u8>, word_list: HashSet<Vec<u8>>) -> i32 {
        let mut que1 = VecDeque::new();
        let mut que2 = VecDeque::new();
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        que1.push_back(s.clone());
        que2.push_back(e.clone());
        map1.insert(s, 0);
        map2.insert(e, 0);
        while !que1.is_empty() && !que2.is_empty() {
            let mut t = -1;
            if que1.len() <= que2.len() {
                t = Self::update(&mut que1, &mut map1, &map2, &word_list);
            } else {
                t = Self::update(&mut que2, &mut map2, &map1, &word_list);
            }
            if t != -1 {
                return t;
            }
        }
        -1
    }

    fn update(
        que: &mut VecDeque<Vec<u8>>,
        cur: &mut HashMap<Vec<u8>, i32>,
        other: &HashMap<Vec<u8>, i32>,
        word_list: &HashSet<Vec<u8>>,
    ) -> i32 {
        let now = que.pop_front().unwrap();
        let path = *cur.get(&now).unwrap();
        let n = now.len();
        for i in 0..n {
            for j in 0..26 {
                let mut sub = now.clone();
                sub[i] = b'a' + j;
                if word_list.contains(&sub) {
                    if cur.contains_key(&sub) {
                        continue;
                    }
                    if other.contains_key(&sub) {
                        return path + 1 + *other.get(&sub).unwrap();
                    } else {
                        que.push_back(sub.clone());
                        cur.insert(sub, path + 1);
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
    ];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
}
