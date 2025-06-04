use crate::Board;
use std::fmt::Display;

use crate::GameState;

pub enum InputType {
    Coord(usize),
    Exit,
    Restart,
    Help,
}

pub enum InputError {
    InvalidCommand,
    InputTooLarge,
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::InvalidCommand => write!(f, "Invalid command. Please try again."),
            InputError::InputTooLarge => {
                write!(f, "That board position does not exist. Try again.")
            }
        }
    }
}

pub trait InputController {
    fn get_raw_input(&self) -> String;
    fn parse_input(&self, input: &str, board_info: &Board) -> Result<InputType, InputError>;
    fn validate_input(&self, input: usize, board_info: &Board) -> Result<(), InputError>;
}

pub trait PlayerController {
    fn handle_input(&self, gamestate: &GameState) -> Result<InputType, InputError>;
}
