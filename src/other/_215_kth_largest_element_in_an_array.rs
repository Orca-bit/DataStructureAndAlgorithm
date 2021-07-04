use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = nums[0..k as usize]
            .iter()
            .map(|&x| Reverse(x))
            .collect::<BinaryHeap<_>>();
        for &num in nums.iter().skip(k as usize) {
            if num > heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.pop().unwrap().0
    }

    pub fn find_kth_largest_1(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut partition = |l: usize, r: usize| -> (usize, usize) {
            nums.swap(l + ((r - l) >> 1), r);
            let mut cur = l;
            let pivot = nums[r];
            let mut less = l as i32 - 1;
            let mut more = r + 1;
            while cur < more {
                if nums[cur] < pivot {
                    nums.swap(cur, (less + 1) as usize);
                    less += 1;
                    cur += 1;
                } else if nums[cur] > pivot {
                    nums.swap(cur, more - 1);
                    more -= 1;
                } else {
                    cur += 1;
                }
            }
            ((less + 1) as usize, more - 1)
        };
        let mut l = 0;
        let mut r = n - 1;
        let k = n - k as usize;
        while l < r {
            let range = partition(l, r);
            if k >= range.0 && k <= range.1 {
                break;
            } else if k < range.0 {
                r = range.0 - 1;
            } else {
                l = range.1 + 1;
            }
        }
        nums[k]
    }
}

#[test]
fn test() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::find_kth_largest(nums, k), res);
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::find_kth_largest_1(nums, k), res);
}
