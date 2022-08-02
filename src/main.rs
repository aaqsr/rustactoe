fn main() {
    let board = [[" "; 3]; 3];

    for row in board {
        for cell in row {
            print!("|{cell}");
        }
        println!("|");
    }
}
