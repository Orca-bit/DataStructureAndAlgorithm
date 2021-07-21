struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let n = board.len() as i32;
        let m = board[0].len() as i32;
        let dx = [-1, -1, -1, 0, 0, 1, 1, 1];
        let dy = [-1, 0, 1, -1, 1, -1, 0, 1];
        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                dx.iter().zip(dy.iter()).for_each(|(&x, &y)| {
                    if i + x >= 0
                        && i + x < n
                        && j + y >= 0
                        && j + y < m
                        && board[(i + x) as usize][(j + y) as usize] & 1 == 1
                    {
                        count += 1;
                    }
                });
                if count == 3 || (board[i as usize][j as usize] == 1 && count == 2) {
                    board[i as usize][j as usize] |= 2;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                board[i as usize][j as usize] >>= 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut board: Vec<Vec<i32>> = vec_vec_i32_1![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
    let res: Vec<Vec<i32>> = vec_vec_i32_1![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, res);
}
