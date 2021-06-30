struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut res = 0;
        while l < r {
            res = res.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height), 49);
}