mod ai;
mod board;
mod controller;
mod player;

use crate::board::*;
use crate::controller::*;
use crate::player::*;

struct GameState<'a> {
    board: Board,
    exit_wanted: bool,
    players: PlayerList<'a>,
}

struct PlayerList<'a> {
    player_1: Player<'a>,
    player_2: Player<'a>,
}

struct PlayerListIterator<'a> {
    players: &'a PlayerList<'a>,
    index: usize,
}

pub struct PlayersInfo {
    ai_piece: u8,
    player_piece: u8,
}

// I am doing this primarily for fun and learning, and so I can iterate over players
impl<'a> Iterator for PlayerListIterator<'a> {
    type Item = &'a Player<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(&self.players.player_1)
            }
            1 => {
                self.index += 1;
                Some(&self.players.player_2)
            }
            _ => {
                self.index = 0;
                None
            }
        }
    }
}

impl<'a> PlayerList<'a> {
    pub fn get_ai_player_piece(&self) -> u8 {
        if self.player_1.player_type() == PlayerType::Local
            || self.player_1.player_type() == PlayerType::Remote
        {
            self.player_2.encoded
        } else {
            self.player_1.encoded
        }
    }

    pub fn get_players_piece_info(&self) -> PlayersInfo {
        PlayersInfo {
            ai_piece: self.get_ai_player_piece(),
            player_piece: self.get_human_player_piece(),
        }
    }

    // Not DRY
    pub fn get_human_player_piece(&self) -> u8 {
        if self.player_1.player_type() == PlayerType::Local
            || self.player_1.player_type() == PlayerType::Remote
        {
            self.player_1.encoded
        } else {
            self.player_2.encoded
        }
    }

    pub fn iter(&self) -> PlayerListIterator {
        PlayerListIterator {
            players: self,
            index: 0,
        }
    }
}

impl GameState<'_> {
    pub fn new<'a>() -> GameState<'a> {
        GameState {
            board: Board::new(),
            players: PlayerList {
                player_1: Player::new("x", 32, PlayerType::Local),
                player_2: Player::new("y", 64, PlayerType::AI(AIStrategy::Minimax)),
            },
            exit_wanted: false,
        }
    }

    fn process_turn(&mut self) {
        for player in self.players.iter() {
            'inputloop: loop {
                match player.controller.handle_input(self) {
                    Ok(InputType::Help) => {
                        self.board.render_help();
                    }
                    Ok(InputType::Coord(coord)) => {
                        let _ = self.board.place(coord, player.encoded);
                        break;
                    }
                    Ok(InputType::Exit) => {
                        self.exit_wanted = true;
                        break;
                    }
                    Ok(_) => print!("Not implemented."),
                    Err(e) => {
                        println!("{e}");
                        continue 'inputloop;
                    }
                }
            }
            if self.exit_wanted {
                break;
            }
        }
        self.check_for_victor();
    }

    fn check_for_victor(&mut self) {
        if let Some(victor_encoded) = self.board.check_for_victory() {
            let mut victor_name: &str = "";
            for p in self.players.iter() {
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
