struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut buy = vec![i32::MIN; k + 1];
        let mut sell = vec![0; k + 1];
        for price in prices {
            buy[0] = buy[0].max(-price);
            for i in 1..=k {
                buy[i] = buy[i].max(sell[i - 1] - price);
                sell[i] = sell[i].max(buy[i] + price);
            }
        }
        sell[k]
    }
}

#[test]
fn test() {
    let prices = vec![2, 4, 1];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_profit(k, prices), res);
    let prices = vec![3, 2, 6, 5, 0, 3];
    let k = 2;
    let res = 7;
    assert_eq!(Solution::max_profit(k, prices), res);
}