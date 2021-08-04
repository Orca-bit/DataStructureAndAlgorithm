struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = image.len();
        for i in 0..n {
            image[i].reverse();
            for j in 0..n {
                image[i][j] = 1 - image[i][j];
            }
        }
        image
    }
}

#[test]
fn test() {
    let a: Vec<Vec<i32>> = vec_vec_i32_1![[1, 1, 0], [1, 0, 1], [0, 0, 0]];
    let b: Vec<Vec<i32>> = vec_vec_i32_1![[1, 0, 0], [0, 1, 0], [1, 1, 1]];
    assert_eq!(Solution::flip_and_invert_image(a), b);
    let a: Vec<Vec<i32>> = vec_vec_i32_1![[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 1], [1, 0, 1, 0]];
    let b: Vec<Vec<i32>> = vec_vec_i32_1![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]];
    assert_eq!(Solution::flip_and_invert_image(a), b);
}
