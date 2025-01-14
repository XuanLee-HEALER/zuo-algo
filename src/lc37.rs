struct Solution;

// Write a program to solve a Sudoku puzzle by filling the empty cells.

// A sudoku solution must satisfy all of the following rules:

// Each of the digits 1-9 must occur exactly once in each row.
// Each of the digits 1-9 must occur exactly once in each column.
// Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
// The '.' character indicates empty cells.

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![vec![0; 10]; 9];
        let mut cols = vec![vec![0; 10]; 9];
        let mut blocks = vec![vec![0; 10]; 9];
        for (i, sub) in board.iter().enumerate() {
            for (j, &v) in sub.iter().enumerate() {
                if v != '.' {
                    let v = (v as u8 - b'0') as usize;
                    rows[i][v] += 1;
                    cols[j][v] += 1;
                    blocks[3 * (i / 3) + j / 3][v] += 1;
                }
            }
        }
        Self::ss(board, 0, 0, &mut rows, &mut cols, &mut blocks);
    }

    fn ss(
        board: &mut [Vec<char>],
        i: usize,
        j: usize,
        rows: &mut [Vec<usize>],
        cols: &mut [Vec<usize>],
        blocks: &mut [Vec<usize>],
    ) -> bool {
        let mut ni = i;
        let mut nj = j;
        let block_num = 3 * (i / 3) + j / 3;
        if j < 8 {
            nj += 1;
        } else {
            ni += 1;
            nj = 0;
        }
        if i == 9 {
            true
        } else if board[i][j] != '.' {
            Self::ss(board, ni, nj, rows, cols, blocks)
        } else {
            let mut res = false;
            for v in 1..=9 {
                if rows[i][v] == 0 && cols[j][v] == 0 && blocks[block_num][v] == 0 {
                    board[i][j] = char::from_digit(v as u32, 10).unwrap();
                    rows[i][v] += 1;
                    cols[j][v] += 1;
                    blocks[block_num][v] += 1;
                    if Self::ss(board, ni, nj, rows, cols, blocks) {
                        res = true;
                        break;
                    } else {
                        board[i][j] = '.';
                        rows[i][v] -= 1;
                        cols[j][v] -= 1;
                        blocks[block_num][v] -= 1;
                    }
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod test_solution {
    #[test]
    fn test_solve_sudoku() {
        let mut sample = vec![
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
        super::Solution::solve_sudoku(&mut sample);
        for sub in sample {
            println!("{:?}", sub)
        }
    }
}
