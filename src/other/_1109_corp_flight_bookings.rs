struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n];
        for b in bookings {
            let first = b[0] as usize - 1;
            let second = b[1] as usize;
            res[first] += b[2];
            if second < n {
                res[second] -= b[2];
            }
        }
        for i in 1..n {
            res[i] += res[i - 1];
        }
        res
    }
}

#[test]
fn test() {
    let bookings = vec_vec_i32_1![[1, 2, 10], [2, 3, 20], [2, 5, 25]];
    let n = 5;
    let res = vec![10, 55, 45, 25, 25];
    assert_eq!(Solution::corp_flight_bookings(bookings, n), res);
}
