use crate::board::*;
use crate::controller::*;
use crate::player::playerlist::*;

pub struct GameState<'a> {
    board: Board,
    exit_wanted: bool,
    players: PlayerList<'a>,
}

impl GameState<'_> {
    pub fn new<'a>() -> GameState<'a> {
        GameState {
            board: Board::new(),
            players: PlayerList::default(),
            exit_wanted: false,
        }
    }

    pub fn game_loop(&mut self) {
        while !self.exit_wanted {
            self.board.render(self);
            self.process_turn();
        }
    }

    pub fn players(&self) -> &PlayerList {
        &self.players
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn process_turn(&mut self) {
        for player in self.players.iter() {
            // Inner loop to ensure player provides correct input
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
                        // We want the player(s) to be able to rectify their choice and provide true input
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
