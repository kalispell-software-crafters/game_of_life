const ROWS: usize = 10; //Operating determined size
const COLUMNS: usize = 10;

fn main() {
    let mut board = [[' '; ROWS]; COLUMNS];
    board[5][5] = '#';
    print_board(&board);
    println!("{}", get_live_neighbors(4, 4, &board));
    println!("{}", get_live_neighbors(0, 0, &board));
}

fn print_board(board: &[[char; ROWS]; COLUMNS]) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
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

#[cfg(test)]
mod tests {
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
}

// Need an initial state
// Alive = #; Dead = ' '

// Rules
// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

// Short Version
// Any live cell with two or three live neighbours survives.
// Any dead cell with three live neighbours becomes a live cell.
// All other live cells die in the next generation. Similarly, all other dead cells stay dead.
