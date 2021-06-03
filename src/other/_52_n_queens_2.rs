struct Solution;

impl Solution {
    fn solve_n_queens_num1(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        let mut record = vec![0; n as usize];
        Self::process1(0, &mut record, n)
    }

    fn process1(i: i32, record: &mut [i32], n: i32) -> i32 {
        if i == n {
            return 1;
        }
        let mut res = 0;
        for j in 0..n {
            if Self::is_valid1(record, i, j) {
                record[i as usize] = j;
                res += Self::process1(i + 1, record, n);
            }
        }
        res
    }

    fn is_valid1(record: &mut [i32], i: i32, j: i32) -> bool {
        for k in 0..i as usize {
            if j == record[k] || (j - record[k]).abs() == (i - k as i32).abs() {
                return false;
            }
        }
        true
    }

    fn solve_n_queens_num2(n: i32) -> i32 {
        // 不超过32
        if n < 1 || n > 32 {
            return 0;
        }
        let upper_lim = if n == 32 { -1 } else { (1 << n) - 1 };
        Self::process2(upper_lim, 0, 0, 0)
    }

    fn process2(upper_lim: i32, col_lim: i32, left_dia_lim: i32, right_dia_lim: i32) -> i32 {
        if col_lim == upper_lim {
            return 1;
        }
        let mut res = 0;
        let mut pos = upper_lim & (!(col_lim | left_dia_lim | right_dia_lim));
        let mut most_right_one = 0;
        while pos != 0 {
            most_right_one = pos & (!pos + 1);
            pos -= most_right_one;
            res += Self::process2(
                upper_lim,
                col_lim | most_right_one,
                (left_dia_lim | most_right_one) << 1,
                (right_dia_lim | most_right_one) >> 1,
            );
        }
        res
    }
}

#[test]
fn test() {
    let n = 14;
    // assert_eq!(Solution::solve_n_queens_num1(n), 92);
    assert_eq!(Solution::solve_n_queens_num2(n), 365596);
}
