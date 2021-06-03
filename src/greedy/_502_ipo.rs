use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut sorted_capital = vec![];
        // 将项目重新组织，与位置绑定，方便处理收益
        for (i, &item) in capital.iter().enumerate() {
            sorted_capital.push((item, i));
        }
        // 根据capital的值进行排序，最小的在最后面
        sorted_capital.sort_unstable_by(|&a, &b| b.0.cmp(&a.0));
        let mut res = w;
        // 大根堆处理收益问题
        // res为当前资金
        let mut max_heap = BinaryHeap::new();
        for _ in 0..k {
            while let Some(&(item, i)) = sorted_capital.last() {
                // 当前所需资金最小的项目可做的时候，将其弹出，并将其对应的收益压入大根堆，否则跳过
                if item <= res {
                    sorted_capital.pop();
                    max_heap.push(profits[i]);
                } else {
                    break;
                }
            }
            // 当大根堆有项目可做时，做收益最大的项目，若大根堆为空，说明无项目可做了，直接返回
            if let Some(profit) = max_heap.pop() {
                res += profit;
            } else {
                return res;
            }
        }
        res
    }
}

#[test]
fn test() {
    let k = 2;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];
    let res = 4;
    assert_eq!(
        Solution::find_maximized_capital(k, w, profits, capital),
        res
    );
}
