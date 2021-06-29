struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, mut cost: Vec<i32>) -> i32 {
        let mut res = -1;
        let n = gas.len();
        let mut init = 0;
        for i in 0..n {
            cost[i] = gas[i] - cost[i];
            if cost[i] >= 0 {
                init = i;
            }
        }
        let mut start = init;
        let mut end = if start == n - 1 { 0 } else { start + 1 };
        let mut need = 0;
        let mut rest = 0;
        loop {
            if start != init && start == if end == 0 { n - 1 } else { end - 1 } {
                break;
            }
            if cost[start] < need {
                need -= cost[start];
            } else {
                rest += cost[start] - need;
                need = 0;
                while rest >= 0 && end != start {
                    rest += cost[end];
                    end = if end == n - 1 { 0 } else { end + 1 };
                }
                if rest >= 0 {
                    res = start as i32;
                    break;
                }
            }
            start = if start == 0 { n - 1 } else { start - 1 };
            if start == init {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
}
