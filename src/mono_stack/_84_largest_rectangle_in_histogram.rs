struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut stack = vec![];
        for (i, &height) in heights.iter().enumerate() {
            while !stack.is_empty() && height <= heights[*stack.last().unwrap()] {
                let pop_index = stack.pop().unwrap();
                let left = if stack.is_empty() {
                    -1
                } else {
                    *stack.last().unwrap() as i32
                };
                let new_value = heights[pop_index] * (i as i32 - left - 1);
                res = res.max(new_value);
            }
            stack.push(i);
        }
        while let Some(index) = stack.pop() {
            let left = if stack.is_empty() {
                -1
            } else {
                *stack.last().unwrap() as i32
            };
            let new_value = heights[index] * (heights.len() as i32 - left - 1);
            res = res.max(new_value);
        }
        res
    }
}

#[test]
fn test() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let res = 10;
    assert_eq!(Solution::largest_rectangle_area(heights), res);
    let heights = vec![2, 1, 2];
    let res = 3;
    assert_eq!(Solution::largest_rectangle_area(heights), res);
}
