use std::{thread, time};

const ROWS: usize = 10; //Operating system determined size
const COLUMNS: usize = 10;

fn main() {
    let mut board = [[' '; ROWS]; COLUMNS];
    board[5][4] = '#';
    board[5][5] = '#';
    board[5][6] = '#';
    board[6][5] = '#';
    board[3][2] = '#';
    board[3][3] = '#';
    board[8][7] = '#';
    print_board(&board); 

    for _ in 0..25 {
        board = advance_generation(&board);
        let board_str = board_to_string(&board);
        print!("{}", board_str);
        let sleep_duration = time::Duration::from_secs(1);
        thread::sleep(sleep_duration);
        clear_board_display(&board_str);
    }
}

fn print_board(board: &[[char; ROWS]; COLUMNS]) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
}

fn board_to_string(board: &[[char; ROWS]; COLUMNS]) -> String {
    let mut board_str = String::new();
    for row in board {
        for cell in row {
            board_str.push(*cell);
        }
        board_str.push('\n');
    }

    board_str
}

fn clear_board_display(board_str: &String) {
    let backspaces = "\x08".repeat(board_str.len());
    print!("{}", backspaces)
}

fn get_live_neighbors(x: usize, y: usize, board: &[[char; ROWS]; COLUMNS]) -> usize {
    let mut count = 0;

    let lower_x = if x == 0 { 0 } else { x - 1 };
    let upper_x = if x == COLUMNS - 1 { COLUMNS - 1 } else { x + 1 };

    let lower_y = if y == 0 { 0 } else { y - 1 };
    let upper_y = if y == ROWS - 1 { ROWS - 1 } else { y + 1 };

    for i in lower_x..=upper_x {
        for j in lower_y..=upper_y {
            if i == x && j == y {
                continue;
            }
            if board[i][j] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

fn advance_generation(board: &[[char; ROWS]; COLUMNS]) -> [[char; ROWS]; COLUMNS] {
    let mut new_board = [[' '; ROWS]; COLUMNS];

    for i in 0..COLUMNS {
        for j in 0..ROWS {
            let count = get_live_neighbors(i, j, &board);
            if count < 2 {
                // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                new_board[i][j] = ' ';
            } else if count == 2 {
                // Any live cell with two or three live neighbours lives on to the next generation.
                new_board[i][j] = board[i][j];
            } else if count > 3 {
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                new_board[i][j] = ' ';
            } else if count == 3 {
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                new_board[i][j] = '#';
            }
        }
    }

    return new_board;
}

#[cfg(test)]
mod tests {
    use crate::advance_generation;

    use super::{get_live_neighbors, COLUMNS, ROWS};

    #[test]
    fn test_no_count_self() {
        let mut board = [[' '; ROWS]; COLUMNS];
        board[5][5] = '#';

        let count = get_live_neighbors(5, 5, &board);
        assert_eq!(count, 0, "Should not count self");
    }

    #[test]
    fn test_corners() {
        let mut board = [[' '; ROWS]; COLUMNS];
        board[0][1] = '#';
        board[1][9] = '#';
        board[9][1] = '#';
        board[9][8] = '#';

        let top_left_count = get_live_neighbors(0, 0, &board);
        assert_eq!(top_left_count, 1);

        let top_right_count = get_live_neighbors(9, 9, &board);
        assert_eq!(top_right_count, 1);

        let bottom_left_count = get_live_neighbors(9, 0, &board);
        assert_eq!(bottom_left_count, 1);

        let bottom_right_count = get_live_neighbors(9, 9, &board);
        assert_eq!(bottom_right_count, 1);
    }

    #[test]
    fn should_die_with_one_neighbor() {
        let mut board = [[' '; ROWS]; COLUMNS];

        board[5][5] = '#';
        board[5][6] = '#';

        let new_board = advance_generation(&board);
        assert_eq!(new_board[5][5], ' ', "Should not be alive");
        assert_eq!(new_board[5][6], ' ', "Should not be alive");
    }

    #[test]
    fn should_live_with_three_neighbors() {
        let mut board = [[' '; ROWS]; COLUMNS];

        board[5][4] = '#';
        board[5][5] = '#';
        board[5][6] = '#';
        board[6][5] = '#';

        let new_board = advance_generation(&board);
        assert_eq!(new_board[5][4], '#', "Should be alive");
        assert_eq!(new_board[5][5], '#', "Should be alive");
        assert_eq!(new_board[5][6], '#', "Should be alive");
        assert_eq!(new_board[6][5], '#', "Should be alive");
    }
}

// Need an initial state
// Alive = #; Dead = ' '

// Short Version
// Any live cell with two or three live neighbours survives.
// Any dead cell with three live neighbours becomes a live cell.
// All other live cells die in the next generation. Similarly, all other dead cells stay dead.
