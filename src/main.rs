#[derive(Default)]
struct Board {
    rows: Vec<Vec<u8>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            rows: vec![vec![32, 0, 32], vec![0, 32, 0], vec![0, 0, 32]],
        }
    }

    pub fn render(&self) {
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    32 => print!("X"),
                    _ => print!(" "),
                }
                if i < 2 {
                    print!(" | ");
                }
            }
            println!();
            println!("__|___|___");
        }
    }
}

fn main() {
    println!("Welcome to tic tac toe.");
    let b = Board::new();
    b.render();
}
