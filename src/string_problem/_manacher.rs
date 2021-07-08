struct Manacher;

impl Manacher {
    fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let manacher_str = Self::manacher_string(s.as_str());
        let mut p_arr = vec![1; manacher_str.len()]; // 回文半径数组
        let mut center = 0;
        let mut right = 0;
        let mut longest = 0;
        let mut res_center = 0;
        for i in 0..manacher_str.len() {
            p_arr[i] = if right > i {
                p_arr[2 * center - i].min(right - i)
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
            if longest < p_arr[i] {
                longest = p_arr[i];
                res_center = i;
            }
        }
        // longest - 1 // manacher串最长回文半径 - 1 为原串最长回文长度
        manacher_str[res_center + 1 - longest..res_center + longest]
            .iter()
            .filter(|&&c| c != '#')
            .collect()
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
    assert_eq!(Manacher::longest_palindrome(s), "aaabaaa".to_string());
}
