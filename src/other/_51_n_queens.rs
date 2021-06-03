struct Solution;

impl Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let upper_lim = (1 << n) - 1;
        let mut res = vec![];
        let mut queens = vec![];
        Self::process(n, &mut res, &mut queens, upper_lim, 0, 0, 0);
        res
    }

    fn process(
        n: i32,
        all: &mut Vec<Vec<String>>,
        queens: &mut Vec<i32>,
        upper_lim: i32,
        col_lim: i32,
        left_dia_lim: i32,
        right_dia_lim: i32,
    ) {
        if col_lim == upper_lim {
            let solution = queens
                .iter()
                .map(|&row| {
                    (0..n)
                        .map(|i| if (1 << i) & row > 0 { 'Q' } else { '.' })
                        .collect::<String>()
                })
                .collect();
            all.push(solution);
            return;
        }
        let mut pos = upper_lim & (!(col_lim | left_dia_lim | right_dia_lim));
        let mut most_right_one = 0;
        while pos != 0 {
            most_right_one = pos & (!pos + 1);
            queens.push(most_right_one);
            pos -= most_right_one;
            Self::process(
                n,
                all,
                queens,
                upper_lim,
                col_lim | most_right_one,
                (left_dia_lim | most_right_one) << 1,
                (right_dia_lim | most_right_one) >> 1,
            );
            queens.pop();
        }
    }
}

#[test]
fn test() {
    let n = 4;
    println!("{:?}", Solution::solve_n_queens(n));
}
