struct Solution;

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        let n = strs.len();
        strs.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        for i in 0..n {
            let mut count = 0;
            for j in 0..n {
                if i != j {
                    if !strs[i].is_sub_str_of(strs[j].as_str()) {
                        count += 1;
                    }
                }
            }
            if count == n - 1 {
                return strs[i].len() as i32;
            }
        }
        -1
    }
}

trait IsSubStr {
    fn is_sub_str_of(&self, other: &str) -> bool;
}

impl IsSubStr for String {
    fn is_sub_str_of(&self, other: &str) -> bool {
        let mut iter = self.chars().peekable();
        for c in other.chars() {
            if let Some(&x) = iter.peek() {
                if c == x {
                    iter.next();
                }
            }
        }
        iter.next().is_none()
    }
}

#[test]
fn test() {
    let strs = vec!["aba".to_string(), "cdc".to_string(), "eae".to_string()];
    let res = 3;
    assert_eq!(Solution::find_lu_slength(strs), res);
}
