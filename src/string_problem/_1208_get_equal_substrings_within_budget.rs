struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let n = s.len();
        let diff = s
            .bytes()
            .zip(t.bytes())
            .map(|(a, b)| (a as i32 - b as i32).abs())
            .collect::<Vec<_>>();
        let (mut l, mut r) = (0, 0);
        let mut cost = 0;
        let mut res = 0;
        while r < n {
            cost += diff[r];
            if cost > max_cost {
                cost -= diff[l];
                l += 1;
            } else {
                res = res.max(r - l + 1);
            }
            r += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let t = "cdef".to_string();
    let max_cost = 3;
    let res = 1;
    assert_eq!(Solution::equal_substring(s, t, max_cost), res);
    let s = "abcd".to_string();
    let t = "acde".to_string();
    let max_cost = 0;
    let res = 1;
    assert_eq!(Solution::equal_substring(s, t, max_cost), res);
    assert_eq!(
        Solution::equal_substring("krrgw".to_string(), "zjxss".to_string(), 19),
        2
    );
    assert_eq!(
        Solution::equal_substring(
            "ujteygggjwxnfl".to_string(),
            "nstsenrzttikoy".to_string(),
            43
        ),
        5
    );
}
