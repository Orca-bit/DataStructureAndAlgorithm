use std::cmp::{max, min};

struct Manacher;

impl Manacher {
    fn longest_palindrome(s: &str) -> usize {
        if s.is_empty() {
            return 0;
        }
        let manacher_str = Self::manacher_string(s);
        let mut p_arr = vec![1; manacher_str.len()]; // 回文半径数组
        let mut center = 0;
        let mut right = 0;
        let mut longest = 0;
        for i in 0..manacher_str.len() {
            p_arr[i] = if right > i {
                min(p_arr[2 * center - i], right - i)
            } else {
                1
            };
            while i + p_arr[i] < manacher_str.len() && i as isize - p_arr[i] as isize >= 0 {
                if manacher_str[i + p_arr[i]] == manacher_str[i - p_arr[i]] {
                    p_arr[i] += 1;
                } else {
                    break;
                }
            }
            if i + p_arr[i] > right {
                right = i + p_arr[i];
                center = i;
            }
            longest = max(longest, p_arr[i]);
        }
        longest - 1 // manacher串最长回文半径 - 1 为原串最长回文长度
    }

    fn manacher_string(s: &str) -> Vec<char> {
        let mut res = vec!['#'; s.len() * 2 + 1];
        let mut index = 1;
        for c in s.chars() {
            res[index] = c;
            index += 2;
        }
        res
    }
}

#[test]
fn test() {
    let s = "aaabaaaa".to_string();
    assert_eq!(Manacher::longest_palindrome(&s), 7);
}
