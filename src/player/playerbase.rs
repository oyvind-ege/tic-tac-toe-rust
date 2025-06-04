use crate::ai::minimax::AIMinimax;
use crate::ai::poor::AIPlayer;
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
    pub encoded: u8,
    ptype: PlayerType,
    pub controller: Box<dyn PlayerController>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AIStrategy {
    Minimax,
    Other,
}

pub struct LocalPlayer {}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, encoded: u8, ptype: PlayerType) -> Player<'a> {
        Player {
            name,
            encoded,
            ptype,
            controller: match ptype {
                PlayerType::Local => Box::new(LocalPlayer {}),
                PlayerType::AI(AIStrategy::Minimax) => Box::new(AIMinimax::new()),
                PlayerType::AI(AIStrategy::Other) => Box::new(AIPlayer::new(encoded)),
                PlayerType::Remote => {
                    println!("Multiplayer not supported.");
                    Box::new(AIPlayer::new(encoded))
                }
            },
        }
    }

    pub fn player_type(&self) -> PlayerType {
        self.ptype
    }
}

impl InputController for LocalPlayer {
    fn get_raw_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        input.trim().to_string()
    }

    fn parse_input(&self, input: &str, board_info: &Board) -> Result<InputType, InputError> {
        match input {
            val if val == "help" || val == "Help" || val == "HELP" => Ok(InputType::Help),
            val if val == "exit" || val == "Exit" || val == "EXIT" => Ok(InputType::Exit),
            val if val.parse::<usize>().is_ok() => {
                let parsed_number = val.parse::<usize>().expect("Could not parse input value.");
                self.validate_input(parsed_number, board_info)?;
                Ok(InputType::Coord(parsed_number))
            }

            _ => Err(InputError::InvalidCommand),
        }
    }

    fn validate_input(&self, input: usize, board_info: &Board) -> Result<(), InputError> {
        if input > board_info.len() - 1 {
            return Err(InputError::InputTooLarge);
        }
        Ok(())
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
