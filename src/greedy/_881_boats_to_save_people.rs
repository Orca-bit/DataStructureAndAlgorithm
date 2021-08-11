struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let n = people.len();
        people.sort_unstable_by(|a, b| b.cmp(a));
        let mut res = 0;
        let (mut l, mut r) = (0, n - 1);
        while l <= r {
            res += 1;
            if people[l] + people[r] <= limit {
                r -= 1;
            }
            l += 1;
        }
        res
    }
}

#[test]
fn test() {
    let people = vec![1, 2];
    let limit = 3;
    let res = 1;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
    let people = vec![3, 2, 2, 1];
    let limit = 3;
    let res = 3;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
    let people = vec![3, 5, 3, 4];
    let limit = 5;
    let res = 4;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
}
