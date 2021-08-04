struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut v = vec![];
        for x in nums {
            if let Err(i) = v.binary_search(&x) {
                if i == v.len() {
                    v.push(x);
                } else {
                    v[i] = x;
                }
            }
            if v.len() == 3 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let res = true;
    assert_eq!(Solution::increasing_triplet(nums), res);
    let nums = vec![5, 4, 3, 2, 1];
    let res = false;
    assert_eq!(Solution::increasing_triplet(nums), res);
}
