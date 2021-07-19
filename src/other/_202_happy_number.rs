struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::digit_square_sum(slow);
            fast = Self::digit_square_sum(Self::digit_square_sum(fast));
            if slow == fast {
                break;
            }
        }
        slow == 1
    }

    fn digit_square_sum(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let i = n % 10;
            res += i * i;
            n /= 10;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_happy(19), true);
}
