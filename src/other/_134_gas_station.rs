struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, mut cost: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut init = 0;
        for i in 0..cost.len() {
            cost[i] = gas[i] - cost[i];
            if cost[i] > 0 {
                init = i;
            }
        }
        let get_next = |now: usize| -> usize {
            if now == cost.len() - 1 {
                0
            } else {
                now + 1
            }
        };
        let get_last = |now: usize| -> usize {
            if now == 0 {
                cost.len() - 1
            } else {
                now - 1
            }
        };
        let mut start = init;
        let mut end = get_next(init);
        let mut rest = 0;
        let mut need = 0;
        loop {
            if start != init && start == get_last(end) {
                break;
            }
            if cost[start] < need {
                need -= cost[start];
            } else {
                rest += cost[start] - need;
                need = 0;
                while rest >= 0 && end != start {
                    rest += cost[end];
                    end = get_next(end);
                }
                if rest >= 0 {
                    res = start as i32;
                    break;
                }
            }
            start = get_last(start);
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
