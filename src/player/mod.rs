use crate::ai::AI;
use crate::controllers::*;
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

impl Player<'_> {
    pub fn new(name: &str, encoded: u8, ptype: PlayerType) -> Player {
        Player {
            name,
            encoded,
            controller: match ptype {
                PlayerType::Local => Box::new(LocalPlayer {}),
                PlayerType::AI => Box::new(AI {}),
                PlayerType::Remote => {
                    println!("Multiplayer not supported.");
                    Box::new(AI {})
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

impl PlayerController for LocalPlayer {
    fn handle_input(&self, _: &GameState) -> Option<InputType> {
        println!("What do you want to do?");
        println!("Type a number from 0 to 8 to make your choice.");
        println!("Type 'help' for assistance on how to designate the board.");
        println!("Type 'exit' to quit.");
        self.parse_input(&self.get_raw_input())
    }
}
