struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as usize;
        let mut res = vec![0; num_people];
        let mut cur = 0;
        while candies > 0 {
            for i in 0..num_people {
                cur += 1;
                if cur >= candies {
                    res[i] += candies;
                    candies = 0;
                    break;
                }
                res[i] += cur;
                candies -= cur;
            }
        }
        res
    }
}

#[test]
fn test() {
    let candies = 7;
    let num_people = 4;
    let res = vec![1, 2, 3, 1];
    assert_eq!(Solution::distribute_candies(candies, num_people), res);
    let candies = 10;
    let num_people = 3;
    let res = vec![5, 2, 3];
    assert_eq!(Solution::distribute_candies(candies, num_people), res);
}
