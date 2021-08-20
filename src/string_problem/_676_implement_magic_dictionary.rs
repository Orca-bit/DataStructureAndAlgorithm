use std::collections::HashMap;

#[derive(Default)]
struct MagicDictionary {
    record: HashMap<Vec<char>, u32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary {
            let mut s = s.chars().collect::<Vec<_>>();
            let n = s.len();
            for i in 0..n {
                let tmp = s[i];
                s[i] = '_';
                *self.record.entry(s.clone()).or_default() |= (1 << (tmp as u8 - b'a') as u32);
                s[i] = tmp;
            }
        }
    }

    fn search(&self, search_word: String) -> bool {
        let mut s = search_word.chars().collect::<Vec<_>>();
        let n = s.len();
        for i in 0..n {
            let tmp = s[i];
            s[i] = '_';
            if let Some(&data) = self.record.get(&s) {
                if (1 << (tmp as u8 - b'a') as u32) ^ data != 0 {
                    return true;
                }
            }
            s[i] = tmp;
        }
        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

#[test]
fn test() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    assert_eq!(obj.search("hello".to_string()), false);
    assert_eq!(obj.search("hhllo".to_string()), true);
    assert_eq!(obj.search("hell".to_string()), false);
    assert_eq!(obj.search("leetcoded".to_string()), false);
}
