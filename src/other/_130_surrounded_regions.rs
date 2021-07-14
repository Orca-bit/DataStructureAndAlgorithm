struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();
        if n < 2 || m < 2 {
            return;
        }
        for j in 0..m {
            if board[0][j] == 'O' {
                Self::infect(board, 0, j as i32);
            }
            if board[n - 1][j] == 'O' {
                Self::infect(board, n as i32 - 1, j as i32);
            }
        }
        for i in 0..n {
            if board[i][0] == 'O' {
                Self::infect(board, i as i32, 0);
            }
            if board[i][m - 1] == 'O' {
                Self::infect(board, i as i32, m as i32 - 1);
            }
        }
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == 'F' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn infect(board: &mut Vec<Vec<char>>, row: i32, col: i32) {
        if row < 0
            || row >= board.len() as i32
            || col < 0
            || col >= board[0].len() as i32
            || board[row as usize][col as usize] != 'O'
        {
            return;
        }
        board[row as usize][col as usize] = 'F';
        Self::infect(board, row - 1, col);
        Self::infect(board, row + 1, col);
        Self::infect(board, row, col - 1);
        Self::infect(board, row, col + 1);
    }
}

#[test]
fn test() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let res = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(board, res);
}
