struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        let mut res = vec![];
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                Self::process(&mut board, &mut trie, i, j, &mut res);
            }
        }
        res
    }

    fn process(
        board: &mut Vec<Vec<char>>,
        trie: &mut Trie,
        row: usize,
        col: usize,
        res: &mut Vec<String>,
    ) {
        let c = board[row][col];
        if c == ' ' {
            return;
        }
        if let Some(trie) = trie.children[(c as u8 - b'a') as usize].as_mut() {
            board[row][col] = ' ';
            if trie.end.is_some() {
                res.push(trie.end.take().unwrap());
            }
            let n = board.len();
            let m = board[0].len();
            if row > 0 {
                Self::process(board, trie, row - 1, col, res);
            }
            if row < n - 1 {
                Self::process(board, trie, row + 1, col, res);
            }
            if col > 0 {
                Self::process(board, trie, row, col - 1, res);
            }
            if col < m - 1 {
                Self::process(board, trie, row, col + 1, res);
            }
            board[row][col] = c;
        }
    }
}

#[derive(Clone)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    end: Option<String>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: vec![None; 26],
            end: None,
        }
    }

    fn insert(&mut self, s: String) {
        let mut p = self;
        for c in s.bytes() {
            if p.children[(c - b'a') as usize].is_none() {
                p.children[(c - b'a') as usize] = Some(Box::new(Trie::new()));
            }
            p = p.children[(c - b'a') as usize].as_mut().unwrap();
        }
        p.end = Some(s);
    }
}

#[test]
fn test() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    let mut res = vec!["eat".to_string(), "oath".to_string()];
    let mut ans = Solution::find_words(board, words);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
