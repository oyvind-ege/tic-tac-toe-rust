use crate::board::*;
use crate::controller::*;
use crate::player::base_player::PlayerPiece;
use crate::player::playerlist::*;

pub struct GameState<'a> {
    board: Board,
    referee: GameReferee,
    exit_wanted: bool,
    players: PlayerList<'a>,
}

impl GameState<'_> {
    pub fn new<'a>() -> GameState<'a> {
        GameState {
            board: Board::new(),
            referee: GameReferee::default(),
            players: PlayerList::default(),
            exit_wanted: false,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn referee(&self) -> &GameReferee {
        &self.referee
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

    fn process_turn(&mut self) {
        for player in self.players.iter() {
            // Inner loop to ensure player provides correct input
            'inputloop: loop {
                match player.controller.handle_input(self) {
                    Ok(InputType::Help) => {
                        self.board.render_help();
                    }
                    Ok(InputType::Coord(coord)) => {
                        let _ = self.board.place(coord, player.player_piece);
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
            if self.board.is_full() {
                break;
            }
            if self.exit_wanted {
                break;
            }
        }
        self.check_for_winner();
        self.check_for_draw();
    }

    fn check_for_winner(&mut self) {
        if let Some(winning_piece) = self.referee.adjudicate(&self.board) {
            let mut winner_name: &str = "";
            for p in self.players.iter() {
                if winning_piece == p.player_piece {
                    winner_name = p.name;
                }
            }
            // TODO: Move responsibility for rendering the winner to a separate object
            self.board.render(self);
            println!("{winner_name} is the victor!");
            self.exit_wanted = true;
        }
    }

    fn check_for_draw(&mut self) {
        if self.board.is_full() {
            //TODO: Remove rendering and state-setting from this function to make it "pure"
            println!("A draw.");
            self.exit_wanted = true;
        }
    }
}

#[derive(Default)]
pub(crate) struct GameReferee {}

impl GameReferee {
    /// Checks to see if there is a winner on the Board, and returns the winning piece if so
    // TODO: It would be more semantic to return a Player, rather than PlayerPiece
    pub fn adjudicate(&self, board: &Board) -> Option<PlayerPiece> {
        self.rows_have_winner(board)
            .or_else(|| self.columns_have_winner(board))
            .or_else(|| self.diagonals_have_winner(board))
    }

    fn rows_have_winner(&self, board: &Board) -> Option<PlayerPiece> {
        for row in board.get_all_rows() {
            let winner = self.has_winner(&row);
            if winner.is_some() {
                return winner;
            }
        }
        None
    }

    fn columns_have_winner(&self, board: &Board) -> Option<PlayerPiece> {
        for column in board.get_all_columns() {
            let winner = self.has_winner(&column);
            if winner.is_some() {
                return winner;
            }
        }

        None
    }

    fn diagonals_have_winner(&self, board: &Board) -> Option<PlayerPiece> {
        self.has_winner(&board.get_diagonal(Diagonal::Major))
            .or_else(|| self.has_winner(&board.get_diagonal(Diagonal::Minor)))
    }

    fn has_winner(&self, slice: &[CellState]) -> Option<PlayerPiece> {
        // Just checking to see if the symbol on the first cell is the same as all symbols in all cells
        if let Some(CellState::Player(player_piece)) = slice.first() {
            if slice
                .iter()
                .all(|&board_cell| board_cell == CellState::Player(*player_piece))
            {
                return Some(*player_piece);
            } else {
                return None;
            }
        }
        None
    }
}
