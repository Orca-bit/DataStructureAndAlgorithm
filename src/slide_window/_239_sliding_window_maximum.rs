use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue = VecDeque::with_capacity(k);
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        for i in 0..nums.len() {
            while !queue.is_empty() && nums[*queue.back().unwrap()] <= nums[i] {
                queue.pop_back();
            }
            queue.push_back(i);
            if *queue.front().unwrap() as i32 == i as i32 - k as i32 { // 头部过期也要弹出
                queue.pop_front();
            }
            if i + 1 >= k {
                res.push(nums[*queue.front().unwrap()]);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let res = vec![3, 3, 5, 5, 6, 7];
    assert_eq!(Solution::max_sliding_window(nums, k), res);
}