struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut first_buy = i32::MIN;
        let mut first_sell = 0;
        let mut second_buy = i32::MIN;
        let mut second_sell = 0;
        for price in prices {
            first_buy = first_buy.max(-price);
            first_sell = first_sell.max(first_buy + price);
            second_buy = second_buy.max(first_sell - price);
            second_sell = second_sell.max(second_buy + price);
        }
        second_sell
    }
}

#[test]
fn test() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let res = 6;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![1, 2, 3, 4, 5];
    let res = 4;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![7, 6, 4, 3, 1];
    let res = 0;
    assert_eq!(Solution::max_profit(prices), res);
}