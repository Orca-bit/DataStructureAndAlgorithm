struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut res = 0;
        let mut is_prime = vec![1; n as usize];
        for i in 2..n {
            if is_prime[i as usize] == 1 {
                res += 1;
                if (i as i64 * i as i64) < n as i64 {
                    let mut j = i * i;
                    while j < n {
                        is_prime[j as usize] = 0;
                        j += i;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_primes(10), 4);
}
