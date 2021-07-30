use std::collections::HashMap;

struct Solution;

const MOD: i64 = 1e9 as i64 + 7;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut res = 0_i64;
        for x in deliciousness {
            for i in (0..22).rev() {
                let y = (1 << i) - x;
                if y < 0 {
                    break;
                }
                if let Some(n) = map.get(&y) {
                    res += n;
                    res %= MOD;
                }
            }
            *map.entry(x).or_default() += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let deliciousness = vec![1, 3, 5, 7, 9];
    let res = 4;
    assert_eq!(Solution::count_pairs(deliciousness), res);
    let deliciousness = vec![1, 1, 1, 3, 3, 3, 7];
    let res = 15;
    assert_eq!(Solution::count_pairs(deliciousness), res);
    let deliciousness = vec![
        149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234,
    ];
    let res = 12;
    assert_eq!(Solution::count_pairs(deliciousness), res);
}
