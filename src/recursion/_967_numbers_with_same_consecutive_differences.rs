struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut cur = vec![];
        for i in 0..10 {
            if i == 0 && n != 1 {
                continue;
            }
            cur.push(i);
            Self::dfs(1, &mut res, &mut cur, n, k);
            cur.pop();
        }
        res
    }

    fn dfs(index: i32, res: &mut Vec<i32>, cur: &mut Vec<i32>, n: i32, k: i32) {
        if index == n {
            let mut x = 0;
            for i in 0..n as usize {
                x *= 10;
                x += cur[i];
            }
            res.push(x);
            return;
        }
        for i in 0..10 {
            if (cur[index as usize - 1] - i).abs() == k {
                cur.push(i);
                Self::dfs(index + 1, res, cur, n, k);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 7;
    let res = vec![181, 292, 707, 818, 929];
    assert_eq!(Solution::nums_same_consec_diff(n, k), res);
    let n = 2;
    let k = 1;
    let res = vec![
        10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
    ];
    assert_eq!(Solution::nums_same_consec_diff(n, k), res);
}
