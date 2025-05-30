mod ai;
mod board;
mod controller;
mod logic;
mod player;

use crate::board::*;
use crate::controller::*;
use crate::logic::LogicController;
use crate::player::*;

struct GameState<'a> {
    board: Board,
    logic_controller: LogicController,
    exit_wanted: bool,
    players: Vec<Player<'a>>,
}

impl GameState<'_> {
    pub fn new<'a>() -> GameState<'a> {
        GameState {
            board: Board::new(),
            logic_controller: LogicController {},
            players: vec![
                Player::new("X", 32, PlayerType::Local),
                Player::new("Y", 64, PlayerType::AI),
            ],
            exit_wanted: false,
        }
    }

    fn process_turn(&mut self) {
        for player in &self.players {
            match player.controller.handle_input(self) {
                Some(InputType::Help) => {
                    self.board.render_help();
                }
                Some(InputType::Coord(coord)) => self.board.place(coord, player.encoded),
                Some(InputType::Exit) => {
                    self.exit_wanted = true;
                }
                _ => {
                    print!("");
                }
            }
        }
        self.check_for_victor();
    }

    fn check_for_victor(&mut self) {
        if let Some(victor_encoded) = self.logic_controller.check_for_victory(&self.board) {
            let mut victor_name: &str = "";
            for p in &self.players {
                if victor_encoded == CellState::Player(p.encoded) {
                    victor_name = p.name;
                }
            }
            self.board.render(self);
            println!("{victor_name} is the victor!");
            self.exit_wanted = true;
        }
    }
}

fn main() {
    println!("Welcome to tic tac toe.");
    let mut game = GameState::new();

    while !game.exit_wanted {
        game.board.render(&game);
        game.process_turn();
    }
}
