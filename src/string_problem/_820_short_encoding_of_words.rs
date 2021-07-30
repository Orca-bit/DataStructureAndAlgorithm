struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for w in words {
            trie.insert(w.bytes().rev());
        }
        let mut res = 0;
        trie.dfs(0, &mut res);
        res as i32
    }
}

#[derive(Clone)]
struct Trie {
    next: Vec<Option<Box<Trie>>>,
    end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            next: vec![None; 26],
            end: false,
        }
    }

    pub fn insert<I>(&mut self, iter: I)
    where
        I: Iterator<Item = u8>,
    {
        let mut node = self;
        for c in iter {
            if node.next[(c - b'a') as usize].is_none() {
                node.next[(c - b'a') as usize] = Some(Box::new(Self::new()));
            }
            node = node.next[(c - b'a') as usize].as_mut().unwrap();
        }
        node.end = true;
    }

    pub fn dfs(&self, length: usize, res: &mut usize) {
        let mut is_end = true;
        for n in self.next.iter() {
            if let Some(node) = n.as_ref() {
                is_end = false;
                node.dfs(length + 1, res);
            }
        }
        if is_end {
            *res += length + 1;
        }
    }
}

#[test]
fn test() {
    let words = vec!["time".to_string(), "me".to_string(), "bell".to_string()];
    let res = 10;
    assert_eq!(Solution::minimum_length_encoding(words), res);
}
