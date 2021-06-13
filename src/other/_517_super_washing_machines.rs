use std::cmp::max;

struct Solution;

impl Solution {
    fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let sum: i32 = machines.iter().sum();
        if sum % n as i32 != 0 {
            return -1;
        }
        let aver = sum / n as i32;
        let mut res = i32::MIN;
        let mut left_sum = 0;
        for i in 0..n {
            let left_rest = left_sum - aver * i as i32;
            let right_rest = (sum - left_sum - machines[i]) - aver * ((n - i) as i32 - 1);
            if left_rest < 0 && right_rest < 0 {
                res = max(res, left_rest.abs() + right_rest.abs());
            } else {
                res = max(res, max(left_rest.abs(), right_rest.abs()));
            }
            left_sum += machines[i];
        }
        res
    }
}

#[test]
fn test() {
    let machines = vec![1, 0, 5];
    let res = 3;
    assert_eq!(Solution::find_min_moves(machines), res);
    let machines = vec![0, 3, 0];
    let res = 2;
    assert_eq!(Solution::find_min_moves(machines), res);
    let machines = vec![0, 2, 0];
    let res = -1;
    assert_eq!(Solution::find_min_moves(machines), res);
}