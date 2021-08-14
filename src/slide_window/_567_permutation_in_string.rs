struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut m1, mut m2) = (vec![0; 26], vec![0; 26]);
        let (n1, n2) = (s1.len(), s2.len());
        if n1 > n2 {
            return false;
        }
        let s1 = s1.bytes().collect::<Vec<_>>();
        let s2 = s2.bytes().collect::<Vec<_>>();
        for i in 0..n1 {
            m1[(s1[i] - b'a') as usize] += 1;
            m2[(s2[i] - b'a') as usize] += 1;
        }
        if m1 == m2 {
            return true;
        }
        for i in n1..n2 {
            m2[(s2[i] - b'a') as usize] += 1;
            m2[(s2[i - n1] - b'a') as usize] -= 1;
            if m1 == m2 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    let res = true;
    assert_eq!(Solution::check_inclusion(s1, s2), res);
    let s1 = "ab".to_string();
    let s2 = "eidboaoo".to_string();
    let res = false;
    assert_eq!(Solution::check_inclusion(s1, s2), res);
}
