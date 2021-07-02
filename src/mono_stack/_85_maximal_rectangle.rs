struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut heights = vec![0; matrix[0].len()];
        let mut res = 0;
        for i in 0..matrix.len() {
            for j in 0..heights.len() {
                heights[j] = if matrix[i][j] == '0' {
                    0
                } else {
                    heights[j] + 1
                };
            }
            match Self::max_area(&heights) {
                Some(value) => res = res.max(value),
                _ => panic!("Some Error"),
            }
        }
        res
    }

    fn max_area(heights: &[i32]) -> Option<i32> {
        if heights.is_empty() {
            return Some(0);
        }
        let mut res = 0;
        let mut stack = vec![];
        for (i, &height) in heights.iter().enumerate() {
            while !stack.is_empty() && height <= heights[*stack.last()?] {
                let pop_index = stack.pop()?;
                let left = if stack.is_empty() {
                    -1
                } else {
                    *stack.last()? as i32
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
                *stack.last()? as i32
            };
            let new_value = heights[index] * (heights.len() as i32 - left - 1);
            res = res.max(new_value);
        }
        Some(res)
    }
}

#[test]
fn test() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let res = 6;
    assert_eq!(Solution::maximal_rectangle(matrix), res);
}
