use std::fmt;

fn main() {
    // let mut board = [[' '; 3]; 3];
    // print_board(&board);
    // board[1][2] = 'x';
    // println!();
    // print_board(&board);
    let mut board = Board::new();
    print!("{board}");
}

fn print_board(board: &[[char; 3]; 3]) {
    for row in board {
        for cell in row {
            print!("|{cell}");
        }
        println!("|");
    }
}

struct Board {
    vals: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            vals: [[' '; 3]; 3],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.vals {
            for cell in row {
                write!(f, "|{cell}");
            }
            writeln!(f, "|");
        }
        Ok(()) // TODO: Error Handling
    }
}
