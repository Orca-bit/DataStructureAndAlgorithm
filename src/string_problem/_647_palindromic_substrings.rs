struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let manacher_str = Self::manacher_str(s.as_str());
        let mut res = 0;
        for x in Self::manacher_array(&manacher_str) {
            res += x >> 1;
        }
        res
    }

    fn manacher_array(s: &[char]) -> Vec<i32> {
        let mut p_array = vec![1; s.len()];
        let mut center = -1;
        let mut right = -1;
        for i in 0..s.len() {
            p_array[i] = if right > i as i32 {
                p_array[center as usize * 2 - i].min(right - i as i32)
            } else {
                1
            };
            while i + (p_array[i] as usize) < s.len() && (i as i32) - p_array[i] >= 0 {
                if s[i + (p_array[i] as usize)] == s[i - (p_array[i] as usize)] {
                    p_array[i] += 1;
                } else {
                    break;
                }
            }
            if i as i32 + p_array[i] > right {
                right = i as i32 + p_array[i];
                center = i as i32;
            }
        }
        p_array
    }

    fn manacher_str(s: &str) -> Vec<char> {
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
    let s = "abc".to_string();
    let res = 3;
    assert_eq!(Solution::count_substrings(s), res);
    let s = "aaa".to_string();
    let res = 6;
    assert_eq!(Solution::count_substrings(s), res);
}
