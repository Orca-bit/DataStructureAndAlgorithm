struct Solution;

impl Solution {
    pub fn find_longest_subarray(array: Vec<&str>) -> Vec<&str> {
        let n = array.len();
        let mut m = vec![None; (n << 1) + 1];
        let mut count = 0;
        let mut res = (0, 0);
        for (i, s) in array.iter().enumerate() {
            if s.chars().next().unwrap().is_ascii_digit() {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 && res.1 - res.0 < i {
                res = (0, i + 1);
            } else {
                if let Some(j) = m[(count + n as i32) as usize] {
                    if res.1 - res.0 + 1 < i - j {
                        res = (j + 1, i + 1);
                    }
                } else {
                    m[(count + n as i32) as usize] = Some(i);
                }
            }
        }
        array[res.0..res.1].to_vec()
    }
}

#[test]
fn test() {
    let array = vec![
        "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I", "J", "K",
        "L", "M",
    ];
    let res = vec![
        "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7",
    ];
    assert_eq!(Solution::find_longest_subarray(array), res);
}
