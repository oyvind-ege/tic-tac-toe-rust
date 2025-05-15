#[derive(Default)]
struct Board {
    rows: Vec<Vec<u8>>,
}

trait Render {
    fn render(&self) {}
}

impl Board {
    pub fn new() -> Board {
        // 32 = player X
        // 64 = player Y
        Board {
            rows: vec![vec![32, 0, 32], vec![0, 32, 0], vec![0, 0, 32]],
        }
    }
}

impl Render for Board {
    fn render(&self) {
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    32 => print!("X"),
                    64 => print!("Y"),
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
