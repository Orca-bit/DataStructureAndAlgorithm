use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    path: u32,
    end: u32,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        link.path += 1;
        for c in word.chars() {
            // 如果表里有value，则令link等于它，相当于向下遍历，否则插入default()
            link = link.children.entry(c).or_default();
            link.path += 1;
        }
        link.end += 1;
    }

    fn delete(&mut self, word: String) {
        if Self::search(&self,word.clone()) {
            let mut link = self;
            link.path -= 1;
            for c in word.chars() {
                link =link.children.get_mut(&c).unwrap();
                link.path -= 1;
            }
            link.end -= 1;
        }
    }

    fn search(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        link.end != 0
    }

    fn starts_with(&self, word: String) -> u32 {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return 0;
            }
        }
        link.path
    }
}

#[test]
fn test() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
    assert_eq!(trie.starts_with("app".to_string()), 2);
    trie.delete("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), 1);
}

#[test]
fn test_entry() {
    let word = "liuhao".to_string();
    let mut hash = HashMap::new();
    hash.insert('o', true);
    let mut tool = false;
    for c in word.chars() {
        tool = *hash.entry(c).or_default();
        println!("{}", tool);
    }
}
