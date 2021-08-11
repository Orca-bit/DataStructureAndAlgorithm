struct Solution;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let n = words.len();
        words.sort_unstable_by_key(|s| s.len());
        let mut res = 1;
        let mut dp = vec![1; n];
        for i in 1..n {
            let s = &words[i];
            for j in (0..i).rev() {
                let candidate = &words[j];
                if candidate.len() == s.len() {
                    continue;
                } else if candidate.len() + 1 == s.len() {
                    if Self::valid(candidate.as_str(), s.as_str()) {
                        dp[i] = dp[i].max(dp[j] + 1);
                    }
                } else {
                    break;
                }
            }
            res = res.max(dp[i]);
        }
        res
    }

    fn valid(prev: &str, word: &str) -> bool {
        let n = prev.len();
        let (mut i, mut j) = (0, 0);
        while j < n {
            if word[i..=i] == prev[j..=j] {
                i += 1;
                j += 1;
            } else if i == j {
                i += 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "ba".to_string(),
        "bca".to_string(),
        "bda".to_string(),
        "bdca".to_string(),
    ];
    assert_eq!(Solution::longest_str_chain(words), 4);
    let words: Vec<String> = vec![
        "ksqvsyq".to_string(),
        "ks".to_string(),
        "kss".to_string(),
        "czvh".to_string(),
        "zczpzvdhx".to_string(),
        "zczpzvh".to_string(),
        "zczpzvhx".to_string(),
        "zcpzvh".to_string(),
        "zczvh".to_string(),
        "gr".to_string(),
        "grukmj".to_string(),
        "ksqvsq".to_string(),
        "gruj".to_string(),
        "kssq".to_string(),
        "ksqsq".to_string(),
        "grukkmj".to_string(),
        "grukj".to_string(),
        "zczpzfvdhx".to_string(),
        "gru".to_string(),
    ];
    assert_eq!(Solution::longest_str_chain(words), 7);
}
