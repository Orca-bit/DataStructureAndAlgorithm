struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MIN;
        let mut sell = 0;
        let mut freeze = 0;
        for price in prices {
            buy = buy.max(freeze - price);
            freeze = sell;
            sell = sell.max(buy + price);
        }
        sell
    }
}

#[test]
fn test() {
    let prices: Vec<i32> = vec![1, 2, 3, 0, 2];
    let res = 3;
    assert_eq!(Solution::max_profit(prices), res);
}