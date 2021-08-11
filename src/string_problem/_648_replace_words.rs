struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for s in dictionary {
            trie.insert(s.as_str());
        }
        sentence
            .split_whitespace()
            .map(|s| trie.map(s))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Clone)]
struct Trie {
    next: Vec<Option<Box<Trie>>>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            next: vec![None; 26],
            end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut ptr = self;
        for c in word.bytes() {
            let index = (c - b'a') as usize;
            if ptr.next[index].is_none() {
                ptr.next[index] = Some(Box::new(Trie::new()));
            }
            ptr = ptr.next[index].as_mut().unwrap();
        }
        ptr.end = true;
    }

    fn map<'a>(&self, word: &'a str) -> &'a str {
        let mut ptr = self;
        for (i, c) in word.char_indices() {
            if ptr.end {
                return &word[0..i];
            }
            let index = (c as u8 - b'a') as usize;
            if ptr.next[index].is_some() {
                ptr = ptr.next[index].as_ref().unwrap();
            } else {
                break;
            }
        }
        word
    }
}

#[test]
fn test() {
    let dict = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
    let sentence = "the cattle was rattled by the battery".to_string();
    let res = "the cat was rat by the bat".to_string();
    assert_eq!(Solution::replace_words(dict, sentence), res);
}
