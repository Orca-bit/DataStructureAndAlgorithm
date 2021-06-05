struct Solution;

impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        let n = a.len();
        Self::process(n, a, b, c);
    }
    fn process(n: usize, from: &mut Vec<i32>, mid: &mut Vec<i32>, to: &mut Vec<i32>) {
        if n == 1 {
            to.push(from.pop().unwrap());
            return;
        }
        Self::process(n - 1, from, to, mid);
        Self::process(1, from, mid, to);
        Self::process(n - 1, mid, from, to);
    }
}

#[test]
fn test() {
    let mut a = vec![1, 2, 3];
    let mut b = vec![];
    let mut c = vec![];
    Solution::hanota(&mut a, &mut b, &mut c);
    println!("{:?},{:?},{:?}", a, b, c);
}
