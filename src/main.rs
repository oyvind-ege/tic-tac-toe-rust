mod board;
use crate::board::Board;
use std::io;

struct Game<'a> {
    board: Board,
    input_controller: InputControl,
    exit_wanted: bool,
    player_1: Player<'a>,
    player_2: Player<'a>,
}

struct Player<'a> {
    name: &'a str,
    encoded: u8,
}

enum InputType {
    Coord(usize),
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
        println!("Type a number from 0 to 8 to make your choice.");
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
            val if val.parse::<usize>().is_ok() => {
                Some(InputType::Coord(val.parse::<usize>().unwrap()))
            }

            _ => None,
        }
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
