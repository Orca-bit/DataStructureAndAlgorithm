struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut left = vec![n as i32; n];
        let mut right = vec![n as i32; n];
        for i in 0..n {
            if seats[i] == 1 {
                left[i] = 0;
            } else {
                if i > 0 {
                    left[i] = left[i - 1] + 1;
                }
            }
        }
        for i in (0..n).rev() {
            if seats[i] == 1 {
                right[i] = 0;
            } else {
                if i < n - 1 {
                    right[i] = right[i + 1] + 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            if seats[i] == 0 {
                res = res.max(left[i].min(right[i]));
            }
        }
        res
    }

    fn method_2(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let (mut first, mut last, mut prev) = (None, None, None);
        let mut res = 0;
        for i in 0..n {
            if seats[i] == 1 {
                if first.is_none() {
                    first = Some(i);
                }
                if let Some(j) = prev {
                    res = res.max((i - j) as i32 >> 1);
                }
                prev = Some(i);
                last = Some(i);
            }
        }
        res.max(first.unwrap() as i32)
            .max((n - last.unwrap() - 1) as i32)
    }
}

#[test]
fn test() {
    let seats = vec![1, 0, 0, 0, 1, 0, 1];
    assert_eq!(Solution::max_dist_to_closest(seats), 2);
    let seats = vec![1, 0, 0, 0];
    assert_eq!(Solution::max_dist_to_closest(seats), 3);
    let seats = vec![1, 0, 0, 0, 1, 0, 1];
    assert_eq!(Solution::method_2(seats), 2);
    let seats = vec![1, 0, 0, 0];
    assert_eq!(Solution::method_2(seats), 3);
}
