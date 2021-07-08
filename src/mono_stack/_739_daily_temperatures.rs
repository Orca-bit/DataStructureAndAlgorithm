struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut mono_stack = vec![];
        let mut res = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while !mono_stack.is_empty()
                && temperatures[i] > temperatures[*mono_stack.last().unwrap()]
            {
                let index = mono_stack.pop().unwrap();
                res[index] = (i - index) as i32;
            }
            mono_stack.push(i);
        }
        res
    }
}

#[test]
fn test() {
    let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let res = vec![1, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(Solution::daily_temperatures(t), res);
}
