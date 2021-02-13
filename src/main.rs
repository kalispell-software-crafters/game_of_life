const ROWS: usize = 10;  //Operating determined size
const COLUMNS: usize = 10;

fn main() {
    let mut board = [[' '; ROWS]; COLUMNS];
    board[5][5] = '#';
    print_board(&board);
}

fn print_board(board:&[[char; ROWS]; COLUMNS]) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
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