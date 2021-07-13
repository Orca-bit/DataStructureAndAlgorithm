struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut buy = i32::MIN;
        let mut sell = 0;
        for price in prices {
            buy = buy.max(sell - price);
            sell = sell.max(buy + price - fee);
        }
        sell
    }
}

#[test]
fn test() {
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    let res = 8;
    assert_eq!(Solution::max_profit(prices, fee), res);
}