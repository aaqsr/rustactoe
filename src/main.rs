use std::fmt;

fn main() {
    let mut board = Board::new();
    print!("{board}");

    let coords = Coords::new(1, 2).unwrap();
    board.substitute(&coords, 'x');
    print!("{board}");
}

struct Coords(u8, u8);

impl Coords {
    fn new(x: u8, y: u8) -> Result<Coords, String> {
        if x < 3 && y < 3 {
            Ok(Coords(x, y))
        } else {
            Err("Coords out of bounds".to_string())
        }
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

    fn substitute(&mut self, coords: &Coords, character: char) -> &mut Self {
        let (x, y) = (coords.0 as usize, coords.1 as usize);
        self.vals[y][x] = character;
        self
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.vals {
            for cell in row {
                match write!(f, "|{cell}") {
                    Ok(()) => {}
                    Err(_) => panic!("Cannot print to console"),
                };
            }
            match writeln!(f, "|") {
                Ok(()) => {}
                Err(_) => panic!("Cannot print to console"),
            };
        }
        Ok(())
    }
}
