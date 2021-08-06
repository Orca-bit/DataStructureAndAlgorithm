struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let a = s.chars().collect::<Vec<_>>();
        let b = goal.chars().collect::<Vec<_>>();
        let n = a.len();
        let m = b.len();
        if m != n {
            return false;
        }
        if a == b {
            let mut v = vec![0; 26];
            for &c in a.iter() {
                v[(c as u8 - b'a') as usize] += 1;
                if v[(c as u8 - b'a') as usize] > 1 {
                    return true;
                }
            }
            return false;
        } else {
            let mut v = vec![];
            for i in 0..n {
                if a[i] != b[i] {
                    v.push(i);
                }
            }
            if v.len() == 2 {
                if a[v[0]] == b[v[1]] && a[v[1]] == b[v[0]] {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let a = "ab".to_string();
    let b = "ba".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "ab".to_string();
    let b = "ab".to_string();
    assert_eq!(Solution::buddy_strings(a, b), false);
    let a = "aa".to_string();
    let b = "aa".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "aaaaaaabc".to_string();
    let b = "aaaaaaacb".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "".to_string();
    let b = "aa".to_string();
    assert_eq!(Solution::buddy_strings(a, b), false);
}
