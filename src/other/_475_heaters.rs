struct Solution;

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let dist = |a: i32, b: i32| -> i32 { (a - b).abs() };
        let (n, m) = (houses.len(), heaters.len());
        houses.sort_unstable();
        heaters.sort_unstable();
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        for i in 0..n {
            while j + 1 < m && dist(houses[i], heaters[j + 1]) <= dist(houses[i], heaters[j]) {
                j += 1;
            }
            res = res.max(dist(houses[i], heaters[j]));
        }
        res
    }
}

#[test]
fn test() {
    let houses = vec![1, 2, 3];
    let heaters = vec![2];
    assert_eq!(Solution::find_radius(houses, heaters), 1);
    let houses = vec![1, 2, 3, 4];
    let heaters = vec![1, 4];
    assert_eq!(Solution::find_radius(houses, heaters), 1);
}
