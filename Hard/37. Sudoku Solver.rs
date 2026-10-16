impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtrack(board);
    }
    
    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for ch in '1'..='9' {
                        if Self::is_valid(board, row, col, ch) {
                            board[row][col] = ch;
                            
                            if Self::backtrack(board) {
                                return true;
                            }
                            
                            board[row][col] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
    
    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, ch: char) -> bool {
        
        for i in 0..9 {
            if board[row][i] == ch {
                return false;
            }
        }
        
        // Проверка столбца
        for i in 0..9 {
            if board[i][col] == ch {
                return false;
            }
        }
        
       
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        
        for i in 0..3 {
            for j in 0..3 {
                if board[box_row + i][box_col + j] == ch {
                    return false;
                }
            }
        }
        
        true
    }
}