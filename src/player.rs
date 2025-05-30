use crate::ai::AIPlayer;
use crate::controller::*;
use crate::GameState;
use std::io;

pub enum PlayerType {
    Local,
    AI,
    Remote,
}

pub struct Player<'a> {
    pub name: &'a str,
    pub encoded: u8,
    pub controller: Box<dyn PlayerController>,
}

pub struct LocalPlayer {}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, encoded: u8, ptype: PlayerType) -> Player<'a> {
        Player {
            name,
            encoded,
            controller: match ptype {
                PlayerType::Local => Box::new(LocalPlayer {}),
                PlayerType::AI => Box::new(AIPlayer::new(encoded)),
                PlayerType::Remote => {
                    println!("Multiplayer not supported.");
                    Box::new(AIPlayer::new(encoded))
                }
            },
        }
    }
}

impl InputController for LocalPlayer {
    fn get_raw_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        input.trim().to_string()
    }

    fn parse_input(&self, input: &str) -> Result<InputType, InputError> {
        match input {
            val if val == "help" || val == "Help" || val == "HELP" => Ok(InputType::Help),
            val if val == "exit" || val == "Exit" || val == "EXIT" => Ok(InputType::Exit),
            val if val.parse::<usize>().is_ok() => {
                Ok(InputType::Coord(val.parse::<usize>().unwrap()))
            }

            _ => Err(InputError::InvalidCommand),
        }
    }
}

impl PlayerController for LocalPlayer {
    fn handle_input(&self, _: &GameState) -> Result<InputType, InputError> {
        println!("What do you want to do?");
        println!("Type a number from 0 to 8 to make your choice.");
        println!("Type 'help' for assistance on how to designate the board.");
        println!("Type 'exit' to quit.");
        self.parse_input(&self.get_raw_input())
    }
}
