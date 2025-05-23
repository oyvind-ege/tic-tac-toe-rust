use core::error;
use std::io;

struct Game<'a> {
    board: Board,
    input_controller: InputControl,
    exit_wanted: bool,
    player_1: Player<'a>,
    player_2: Player<'a>,
}

#[derive(Default)]
struct Board {
    rows: Vec<Vec<u8>>,
    number_of_columns: usize,
}

struct Player<'a> {
    name: &'a str,
    encoded: u8,
}

struct Coordinate {
    x: usize,
    y: usize,
}

enum InputType {
    Coord(Coordinate),
    Exit,
    Help,
}

struct InputControl {}

impl Game<'_> {
    pub fn new<'a>() -> Game<'a> {
        Game {
            board: Board::new(),
            input_controller: InputControl {},
            player_1: Player {
                name: "X",
                encoded: 32,
            },
            player_2: Player {
                name: "Y",
                encoded: 64,
            },
            exit_wanted: false,
        }
    }

    pub fn handle_input(&mut self) {
        println!("What do you want to do?");
        println!("Type a number from 1 to 9 to make your choice.");
        println!("Type 'help' for assistance on how to designate the board.");
        println!("Type 'exit' to quit.");
        let input = self
            .input_controller
            .parse_input(&self.input_controller.get_raw_input());

        match input {
            Some(inp) => match inp {
                InputType::Help => self.board.render_help(),
                //TODO: Add support for two players?
                InputType::Coord(coord) => self.board.place(coord, self.player_1.encoded),
                InputType::Exit => self.exit_wanted = true,
            },
            None => println!("Incorrect input"),
        }
    }

    pub fn check_for_victor(&mut self) {
        if let Some(v) = self.board.check_for_victory() {
            let mut player: &str = "";
            match v {
                val if val == self.player_1.encoded => player = self.player_1.name,
                val if val == self.player_2.encoded => player = self.player_2.name,
                _ => println!("Strange...Game has proclaimed a victor that does no exist."),
            }
            self.board.render(self);
            println!("{} is the victor!", player);
            self.exit_wanted = true;
        }
    }
}

impl InputControl {
    fn get_raw_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        input.trim().to_string()
    }

    fn parse_input(&self, input: &str) -> Option<InputType> {
        match input {
            val if val == "help" || val == "Help" || val == "HELP" => Some(InputType::Help),
            val if val == "exit" || val == "Exit" || val == "EXIT" => Some(InputType::Exit),
            val if val.parse::<u8>().is_ok() => match self.get_coords_from_input(val) {
                Ok(coord) => Some(InputType::Coord(coord)),
                Err(_) => None,
            },
            _ => None,
        }
    }

    fn get_coords_from_input(&self, string: &str) -> Result<Coordinate, Box<dyn error::Error>> {
        let num = string.parse::<u8>()?;
        match num {
            1 => Ok(Coordinate { x: 0, y: 0 }),
            2 => Ok(Coordinate { x: 1, y: 0 }),
            3 => Ok(Coordinate { x: 2, y: 0 }),
            4 => Ok(Coordinate { x: 0, y: 1 }),
            5 => Ok(Coordinate { x: 1, y: 1 }),
            6 => Ok(Coordinate { x: 2, y: 1 }),
            7 => Ok(Coordinate { x: 0, y: 2 }),
            8 => Ok(Coordinate { x: 1, y: 2 }),
            9 => Ok(Coordinate { x: 2, y: 2 }),
            _ => Err("Invalid number. Try again.".into()),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            rows: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            number_of_columns: 3,
        }
    }

    pub fn place(&mut self, coordinate: Coordinate, symbol_to_place: u8) {
        self.rows[coordinate.y][coordinate.x] = symbol_to_place;
    }

    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self) -> Option<u8> {
        self.has_horizontal_victor()
            .or_else(|| self.has_vertical_victor())
            .or_else(|| self.has_diagonal_victor())
    }

    fn has_horizontal_victor(&self) -> Option<u8> {
        for row in &self.rows {
            if let Some(victor) = self.has_victor(row) {
                return Some(victor);
            }
        }
        None
    }

    fn has_vertical_victor(&self) -> Option<u8> {
        let mut verticals: Vec<Vec<u8>> = vec![vec![], vec![], vec![]];

        for i in 0..self.number_of_columns {
            verticals.push(self.get_vertical(i));
        }

        for v in verticals {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self) -> Option<u8> {
        let diagonal_1 = self.get_diagonal(1);
        let diagonal_2 = self.get_diagonal(2);

        self.has_victor(&diagonal_1)
            .or_else(|| self.has_victor(&diagonal_2))
    }

    fn has_victor(&self, vec: &[u8]) -> Option<u8> {
        let first = vec.first();
        first?;
        if *first.unwrap() == 0 {
            return None;
        }
        if vec.iter().all(|&x| x == *first.unwrap()) {
            Some(*first.unwrap())
        } else {
            None
        }
    }

    /// Returns a new vector consisting of a "vertical" column in the board
    /// * n represents the column number, from the right
    fn get_vertical(&self, n: usize) -> Vec<u8> {
        self.rows.iter().map(|row| row[n]).collect()
    }

    fn get_diagonal(&self, n: usize) -> Vec<u8> {
        self.rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                let mut column_index = row_index;
                if n == 2 {
                    column_index = 2 - row_index;
                }
                row[column_index]
            })
            .collect()
    }

    fn render(&self, game: &Game) {
        println!();
        println!("The board currently looks like this:");
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                match *cell {
                    val if val == game.player_1.encoded => print!("{}", game.player_1.name),
                    val if val == game.player_2.encoded => print!("{}", game.player_2.name),
                    _ => print!(" "),
                }
                if i < 2 {
                    print!(" | ");
                }
            }
            println!();
            println!("__|___|___");
        }
        println!();
    }

    fn render_help(&self) {
        println!();
        println!("This is how you designate the board cells:");
        for (row_index, row) in self.rows.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                print!("{}", (row_index * self.number_of_columns + col_index) + 1);
                if col_index < 2 {
                    print!(" | ");
                }
            }
            println!();
            println!("__|___|___");
        }
        println!();
    }
}

fn main() {
    println!("Welcome to tic tac toe.");

    let mut game = Game::new();

    while !game.exit_wanted {
        game.board.render(&game);
        game.handle_input();
        game.check_for_victor();
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn basic() {
        let b = Board::new();
        let row = vec![32, 32, 32];
        assert_eq!(b.has_victor(&row), Some(32));
    }

    #[test]
    fn basic_wrong() {
        let b = Board::new();
        let row = vec![0, 32, 32];
        assert_eq!(b.has_victor(&row), None);
    }

    #[test]
    fn whatever() {
        let mut b = Board::new();
        b.rows = vec![vec![32, 32, 0], vec![0, 0, 0], vec![64, 2, 0]];
        assert_eq!(b.has_horizontal_victor(), None);
    }

    #[test]
    fn whatever2() {
        let mut b = Board::new();
        b.rows = vec![vec![32, 32, 0], vec![64, 64, 64], vec![64, 2, 0]];
        assert_eq!(b.has_horizontal_victor(), Some(64));
    }
}
