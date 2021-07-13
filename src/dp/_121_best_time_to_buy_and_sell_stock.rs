struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MIN;
        let mut sell = 0;
        for price in prices {
            buy = buy.max(-price);
            sell = sell.max(buy + price);
        }
        sell
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