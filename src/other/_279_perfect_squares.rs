struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut res = n;
        let mut num = 2_i32;
        while num.pow(2) <= n {
            res = res.min(n / num.pow(2) + Self::num_squares(n % num.pow(2)));
            num += 1;
        }
        res
    }

    fn num_squares_1(n: i32) -> i32 {
        let mut rest = n;
        while rest % 4 == 0 {
            rest /= 4;
        }
        if rest % 8 == 7 || n % 8 == 7 {
            return 4;
        }
        let sqr = f64::sqrt(n as f64);
        if sqr as i32 * sqr as i32 == n {
            return 1;
        }
        let mut i = 1;
        while i * i <= n / 2 {
            let j = f64::sqrt((n - i * i) as f64) as i32;
            if i * i + j * j == n {
                return 2;
            }
            i += 1;
        }
        3
    }
}

#[test]
fn test() {
    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut v3 = vec![];
    let mut v4 = vec![];
    for i in 1..=100 {
        let res = Solution::num_squares(i);
        assert_eq!(res, Solution::num_squares_1(i));
        match res {
            1 => v1.push(i),
            2 => v2.push(i),
            3 => v3.push(i),
            4 => v4.push(i),
            _ => panic!(),
        }
    }
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
}
