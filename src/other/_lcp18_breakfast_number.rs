struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn breakfast_number(mut staple: Vec<i32>, mut drinks: Vec<i32>, x: i32) -> i32 {
        let mut res = 0;
        staple.sort_unstable();
        drinks.sort_unstable();
        let (n, m) = (staple.len(), drinks.len());
        let (mut i, mut j) = (0, m as i32 - 1);
        while i < n && j >= 0 {
            if staple[i] + drinks[j as usize] <= x {
                res += j + 1;
                res %= MOD;
                i += 1;
            } else {
                j -= 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::breakfast_number(vec![10, 20, 5], vec![5, 5, 2], 15),
        6
    );
    assert_eq!(
        Solution::breakfast_number(vec![2, 1, 1], vec![8, 9, 5, 1], 9),
        8
    );
}
