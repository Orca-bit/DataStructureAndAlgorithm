struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::process(&mut board, &word, 0, i as i32, j as i32) {
                    return true;
                }
            }
        }
        false
    }

    fn process(
        board: &mut Vec<Vec<char>>,
        word: &[char],
        index: usize,
        row: i32,
        col: i32,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        if row < 0 || row >= board.len() as i32 || col < 0 || col >= board[0].len() as i32 {
            return false;
        }
        if board[row as usize][col as usize] != word[index] {
            return false;
        }
        let tmp = board[row as usize][col as usize];
        board[row as usize][col as usize] = ' ';
        let res = Self::process(board, word, index + 1, row - 1, col)
            || Self::process(board, word, index + 1, row + 1, col)
            || Self::process(board, word, index + 1, row, col - 1)
            || Self::process(board, word, index + 1, row, col + 1);
        board[row as usize][col as usize] = tmp;
        res
    }
}

#[test]
fn test() {
    let board = vec![];
    let word = "AC".to_string();
    assert_eq!(Solution::exist(board, word), false);
    let board = vec![vec!['A', 'C'], vec!['A', 'D']];
}
