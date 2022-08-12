use std::fmt;

fn main() {
    let mut board = Board::new();
    print!("{board}");

    // let coords = Coords::new(1, 2).unwrap();
    // board.substitute(&coords, 'x');
    // print!("{board}");
}

// struct Coords(u8, u8);

// impl Coords {
//     fn new(x: u8, y: u8) -> Result<Coords, String> {
//         if x < 3 && y < 3 {
//             Ok(Coords(x, y))
//         } else {
//             Err("Coords out of bounds".to_string())
//         }
//     }
// }

struct Board {
    vals: [char; 9],
}

impl Board {
    fn new() -> Board {
        Board { vals: [' '; 9] }
    }

    // fn substitute(&mut self, character: char) -> &mut Self {
    //     let (x, y) = (coords.0 as usize, coords.1 as usize);
    //     self.vals[y][x] = character;
    //     self
    // }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.vals.len() {
            let cell = self.vals[i];
            if i % 3 == 0 && i != 0 {
                match writeln!(f, "|") {
                    // match writeln!(f, "|") {
                    Ok(()) => {}
                    Err(_) => panic!("Cannot print to console"),
                };
            }
            match write!(f, "|{cell}") {
                // match writeln!(f, "|") {
                Ok(()) => {}
                Err(_) => panic!("Cannot print to console"),
            };
        }
        match write!(f, "|") {
            Ok(()) => {}
            Err(_) => panic!("Cannot print to console"),
        }
        Ok(())
    }
}
