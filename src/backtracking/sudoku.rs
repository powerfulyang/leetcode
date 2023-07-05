use std::collections::HashSet;

fn dfs(
    board: &mut Vec<Vec<char>>,
    rows: &mut Vec<HashSet<usize>>,
    cols: &mut Vec<HashSet<usize>>,
    boxes: &mut Vec<HashSet<usize>>,
    spaces: &mut Vec<(usize, usize)>,
    pos: usize,
) -> bool {
    if pos == spaces.len() {
        return true;
    }
    let (i, j) = spaces[pos];
    for n in 1..=9 {
        if rows[i].contains(&n) || cols[j].contains(&n) || boxes[(i / 3) * 3 + j / 3].contains(&n) {
            continue;
        }
        board[i][j] = (n as u8 + b'0') as char;
        rows[i].insert(n);
        cols[j].insert(n);
        boxes[(i / 3) * 3 + j / 3].insert(n);
        if dfs(board, rows, cols, boxes, spaces, pos + 1) {
            return true;
        }
        rows[i].remove(&n);
        cols[j].remove(&n);
        boxes[(i / 3) * 3 + j / 3].remove(&n);
    }
    false
}

// 数独
#[allow(dead_code)]
fn sudoku(board: &mut Vec<Vec<char>>) {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];
    let mut spaces = vec![];
    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                spaces.push((i, j));
            } else {
                let n = (c as u8 - b'0') as usize;
                rows[i].insert(n);
                cols[j].insert(n);
                boxes[(i / 3) * 3 + j / 3].insert(n);
            }
        }
    }
    dfs(board, &mut rows, &mut cols, &mut boxes, &mut spaces, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku1() {
        let mut board = vec![
            vec!['.', '.', '.', '7', '.', '.', '1', '.', '.'],
            vec!['7', '.', '8', '.', '.', '.', '9', '.', '.'],
            vec!['.', '2', '.', '.', '4', '6', '5', '.', '.'],
            vec!['.', '.', '7', '.', '9', '1', '.', '3', '.'],
            vec!['9', '.', '.', '.', '2', '7', '.', '.', '4'],
            vec!['.', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '2', '4', '9', '.'],
            vec!['5', '6', '.', '.', '7', '4', '.', '.', '.'],
            vec!['.', '7', '.', '.', '.', '.', '.', '.', '.'],
        ];
        sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['6', '9', '5', '7', '3', '8', '1', '4', '2'],
                vec!['7', '4', '8', '2', '1', '5', '9', '6', '3'],
                vec!['3', '2', '1', '9', '4', '6', '5', '7', '8'],
                vec!['2', '8', '7', '4', '9', '1', '6', '3', '5'],
                vec!['9', '3', '6', '5', '2', '7', '8', '1', '4'],
                vec!['1', '5', '4', '8', '6', '3', '7', '2', '9'],
                vec!['8', '1', '3', '6', '5', '2', '4', '9', '7'],
                vec!['5', '6', '9', '3', '7', '4', '2', '8', '1'],
                vec!['4', '7', '2', '1', '8', '9', '3', '5', '6'],
            ]
        );
    }

    #[test]
    fn test_sudoku2() {
        let mut board = vec![
            vec!['7', '.', '9', '.', '.', '4', '.', '1', '.'],
            vec!['.', '.', '3', '7', '.', '2', '.', '9', '8'],
            vec!['1', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '1', '9', '.', '2'],
            vec!['.', '.', '.', '8', '6', '.', '.', '3', '.'],
            vec!['.', '.', '.', '.', '4', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '8', '.', '.', '1'],
            vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
            vec!['4', '2', '7', '.', '.', '.', '.', '.', '5'],
        ];
        sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['7', '8', '9', '3', '5', '4', '2', '1', '6'],
                vec!['5', '6', '3', '7', '1', '2', '4', '9', '8'],
                vec!['1', '4', '2', '9', '8', '6', '5', '7', '3'],
                vec!['6', '7', '4', '5', '3', '1', '9', '8', '2'],
                vec!['2', '9', '5', '8', '6', '7', '1', '3', '4'],
                vec!['3', '1', '8', '2', '4', '9', '6', '5', '7'],
                vec!['9', '5', '6', '4', '7', '8', '3', '2', '1'],
                vec!['8', '3', '1', '6', '2', '5', '7', '4', '9'],
                vec!['4', '2', '7', '1', '9', '3', '8', '6', '5'],
            ]
        );
    }
}
