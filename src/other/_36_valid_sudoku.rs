struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = vec![vec![false; 10]; 9];
        let mut col = vec![vec![false; 10]; 9];
        let mut cube = vec![vec![false; 10]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let cude_no = 3 * (i / 3) + j / 3;
                if board[i][j] != '.' {
                    let index = (board[i][j] as u8 - b'0') as usize;
                    if row[i][index] == true
                        || col[j][index] == true
                        || cube[cude_no][index] == true
                    {
                        return false;
                    }
                    row[i][index] = true;
                    col[j][index] = true;
                    cube[cude_no][index] = true;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), true);
    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
}
