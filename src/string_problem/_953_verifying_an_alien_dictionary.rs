struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = vec![0; 26];
        for (i, c) in order.char_indices() {
            map[(c as u8 - b'a') as usize] = b'a' + i as u8;
        }
        let words = words
            .into_iter()
            .map(|s| {
                s.bytes()
                    .map(|c| map[(c - b'a') as usize])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut sorted = words.clone();
        sorted.sort_unstable();
        sorted == words
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec!["hello".to_string(), "leetcode".to_string()];
    let order: String = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);

    let words: Vec<String> = vec!["word".to_string(), "world".to_string(), "row".to_string()];
    let order: String = "worldabcefghijkmnpqstuvxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec!["apple".to_string(), "app".to_string()];
    let order: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec!["kuvp".to_string(), "q".to_string()];
    let order: String = "ngxlkthsjuoqcpavbfdermiywz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);
}
