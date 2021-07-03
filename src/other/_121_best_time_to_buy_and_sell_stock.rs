struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min = prices[0];
        for &price in prices.iter().skip(1) {
            res = res.max(price - min);
            min = min.min(price);
        }
        res
    }
}

#[test]
fn test() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let res = 5;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![7, 6, 4, 3, 1];
    let res = 0;
    assert_eq!(Solution::max_profit(prices), res);
}