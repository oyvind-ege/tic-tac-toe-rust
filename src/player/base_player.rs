use crate::ai::minimax::AIMinimax;
use crate::ai::AIStrategy;
use crate::board::Board;
use crate::controller::*;
use crate::GameState;
use std::io;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PlayerType {
    Local,
    AI(AIStrategy),
    Remote,
}

pub struct Player<'a> {
    pub name: &'a str,
    pub player_piece: u8,
    player_type: PlayerType,
    pub controller: Box<dyn PlayerController>,
}

pub struct LocalPlayer {}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, player_piece: u8, player_type: PlayerType) -> Player<'a> {
        Player {
            name,
            player_piece,
            player_type,
            controller: match player_type {
                PlayerType::Local => Box::new(LocalPlayer {}),
                PlayerType::AI(AIStrategy::Minimax) => Box::new(AIMinimax::new()),
                PlayerType::Remote => {
                    println!("Multiplayer not supported.");
                    Box::new(AIMinimax::new())
                }
            },
        }
    }

    pub fn player_type(&self) -> PlayerType {
        self.player_type
    }
}

impl InputController for LocalPlayer {
    fn get_raw_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        input.trim().to_string()
    }

    fn parse_input(&self, input: &str, board_info: &Board) -> Result<InputType, InputError> {
        match input.to_lowercase().as_str() {
            "help" => Ok(InputType::Help),
            "exit" => Ok(InputType::Exit),
            val if val.parse::<usize>().is_ok() => {
                let parsed_number = val.parse::<usize>().expect("Could not parse input value.");

                match board_info.is_valid_move(parsed_number) {
                    Ok(_) => Ok(InputType::Coord(parsed_number)),
                    Err(e) => Err(InputError::InvalidBoardError(e)),
                }
            }
            _ => Err(InputError::InvalidCommand),
        }
    }
}

impl PlayerController for LocalPlayer {
    fn handle_input(&self, game_state: &GameState) -> Result<InputType, InputError> {
        println!();
        println!("What do you want to do?");
        println!("Type a number from 0 to 8 to make your choice.");
        println!("Type 'help' for assistance on how to designate the board.");
        println!("Type 'exit' to quit.");
        self.parse_input(&self.get_raw_input(), game_state.board())
    }
}
