fn is_scramble(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }
    let s1 = s1.trim().chars().collect::<Vec<_>>();
    let s2 = s2.trim().chars().collect::<Vec<_>>();
    let mut s3 = s1.clone();
    let mut s4 = s2.clone();
    s3.sort_unstable();
    s4.sort_unstable();
    if s3 != s4 {
        return false;
    }
    let n = s1.len();
    let mut dp = vec![vec![vec![None; n + 1]; n]; n];
    process(&s1, &s2, 0, 0, n, &mut dp)
}

fn process(
    s1: &[char],
    s2: &[char],
    left1: usize,
    left2: usize,
    size: usize,
    dp: &mut Vec<Vec<Vec<Option<bool>>>>,
) -> bool {
    if let Some(item) = dp[left1][left2][size] {
        return item;
    }
    let mut res = false;
    if size == 1 {
        res = s1[left1] == s2[left2];
    } else {
        for i in 1..size {
            if (process(s1, s2, left1, left2, i, dp)
                && process(s1, s2, left1 + i, left2 + i, size - i, dp))
                || (process(s1, s2, left1, left2 + size - i, i, dp)
                    && process(s1, s2, left1 + i, left2, size - i, dp))
            {
                res = true;
                break;
            }
        }
    }
    dp[left1][left2][size] = Some(res);
    res
}

#[test]
fn test() {
    assert_eq!(is_scramble("abcd".to_string(), "cadb".to_string()), false);
    assert_eq!(is_scramble("abcd".to_string(), "dbac".to_string()), true)
}
