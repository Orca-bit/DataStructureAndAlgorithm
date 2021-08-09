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

    fn insert(&mut self, word: String) {
        let mut ptr = self;
        for c in word.bytes().rev() {
            if ptr.next[(c - b'a') as usize].is_none() {
                ptr.next[(c - b'a') as usize] = Some(Box::new(Trie::new()));
            }
            ptr = ptr.next[(c - b'a') as usize].as_mut().unwrap();
        }
        ptr.end = true;
    }

    fn search(&self, stream: &[u8]) -> bool {
        let mut ptr = self;
        for c in stream.iter().rev() {
            if ptr.next[(c - b'a') as usize].is_some() {
                ptr = ptr.next[(c - b'a') as usize].as_ref().unwrap();
                if ptr.end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie,
    stream: Vec<u8>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        Self {
            trie,
            stream: vec![],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);
        self.trie.search(&self.stream)
    }
}

#[test]
fn test() {
    let words = vec!["cd".to_string(), "f".to_string(), "kl".to_string()];
    let mut obj = StreamChecker::new(words);
    assert_eq!(obj.query('a'), false);
    assert_eq!(obj.query('b'), false);
    assert_eq!(obj.query('c'), false);
    assert_eq!(obj.query('d'), true);
    assert_eq!(obj.query('e'), false);
    assert_eq!(obj.query('f'), true);
    assert_eq!(obj.query('g'), false);
    assert_eq!(obj.query('h'), false);
    assert_eq!(obj.query('i'), false);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('k'), false);
    assert_eq!(obj.query('l'), true);
}
