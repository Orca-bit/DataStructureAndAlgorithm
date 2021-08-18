struct Solution;

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut res = (-1, vec![0; 4]);
        for i in 0..4 {
            for j in 0..4 {
                if j != i {
                    for k in 0..4 {
                        if k != i && k != j {
                            let l = 6 - i - j - k;
                            let hour = arr[i] * 10 + arr[j];
                            let minute = arr[k] * 10 + arr[l];
                            if hour < 24 && minute < 60 && res.0 < hour * 60 + minute {
                                res.0 = hour * 60 + minute;
                                res.1 = vec![arr[i], arr[j], arr[k], arr[l]];
                            }
                        }
                    }
                }
            }
        }
        if res.0 == -1 {
            "".to_string()
        } else {
            format!("{}{}:{}{}", res.1[0], res.1[1], res.1[2], res.1[3])
        }
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4];
    let res = "23:41".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
    let a = vec![5, 5, 5, 5];
    let res = "".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
    let a = vec![0, 0, 0, 0];
    let res = "00:00".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
}
