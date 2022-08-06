use std::fmt;

fn main() {
    let mut board = Board::new();
    print!("{board}");
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
