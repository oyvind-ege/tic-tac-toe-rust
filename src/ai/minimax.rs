use crate::PlayersInfo;
use std::cmp;

use crate::board::CellState;
use crate::controller::InputError;
use crate::InputType;
use crate::{Board, GameState, PlayerController};

pub struct AIMinimax {}

impl PlayerController for AIMinimax {
    fn handle_input(&self, gamestate: &GameState) -> Result<InputType, InputError> {
        let players_info = gamestate.players.get_players_piece_info();
        let minimax = self.minimax(
            0,
            &gamestate.board,
            &players_info,
            gamestate.players.get_ai_player_piece(),
            true,
        );
        Ok(InputType::Coord(minimax.unwrap().0))
    }
}

/// Represents (move, score) tuple, where move is usize and score (i8) is the score of that move
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct MiniMaxMoveAndScore(usize, i8);

impl Ord for MiniMaxMoveAndScore {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for MiniMaxMoveAndScore {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AIMinimax {
    pub fn new() -> AIMinimax {
        AIMinimax {}
    }

    fn minimax(
        &self,
        last_move: usize,
        board: &Board,
        players_info: &PlayersInfo,
        piece_to_move: u8,
        is_maximizer: bool,
    ) -> Option<MiniMaxMoveAndScore> {
        const DEFAULT_NEGATIVE_SCORE: i8 = -128;
        const DEFAULT_POSITIVE_SCORE: i8 = 127;
        const WINNING_MOVE_SCORE: i8 = 10;
        const LOSING_MOVE_SCORE: i8 = -10;
        const DRAW_MOVE_SCORE: i8 = 0;

        let next_piece_to_move = if piece_to_move == players_info.ai_piece {
            players_info.player_piece
        } else {
            players_info.ai_piece
        };

        let victor = board.check_for_victory();
        if victor.is_some() || board.is_full() {
            if let Some(winning_piece) = victor {
                // find out winner
                if winning_piece == CellState::Player(players_info.ai_piece) {
                    return Some(MiniMaxMoveAndScore(last_move, WINNING_MOVE_SCORE));
                } else {
                    return Some(MiniMaxMoveAndScore(last_move, LOSING_MOVE_SCORE));
                }
            }
            return Some(MiniMaxMoveAndScore(last_move, DRAW_MOVE_SCORE));
        }

        let possible_state_list =
            self.get_possible_states_from_state(board, CellState::Player(next_piece_to_move));

        if possible_state_list.is_empty() {
            return None;
        }

        let mut best;

        if is_maximizer {
            best = MiniMaxMoveAndScore(last_move, DEFAULT_NEGATIVE_SCORE);
            for (state, attempted_move) in possible_state_list {
                // state must here be a board with a given move already populated
                // TODO: Add (Board, Move) tuple and make this the return value of get_possible_states
                let new_best = self.minimax(
                    attempted_move,
                    &state,
                    players_info,
                    next_piece_to_move,
                    false,
                );

                best = cmp::max(best, new_best?)
            }
        } else {
            best = MiniMaxMoveAndScore(last_move, DEFAULT_POSITIVE_SCORE);
            for (state, attempted_move) in possible_state_list {
                best = cmp::min(
                    best,
                    self.minimax(
                        attempted_move,
                        &state,
                        players_info,
                        next_piece_to_move,
                        true,
                    )?,
                )
            }
        }
        dbg!("Best move is {}", best.0);
        Some(best)
    }

    fn get_possible_states_from_state(
        &self,
        from_state: &Board,
        player_to_move: CellState,
    ) -> Vec<(Board, usize)> {
        let mut possible_states: Vec<(Board, usize)> = vec![];
        let empties = from_state.get_positions_of_empty_cells();
        for empty_cell_index in empties {
            let mut new_state = from_state.clone();
            new_state.modify_at_cell(empty_cell_index, player_to_move);
            possible_states.push((new_state, empty_cell_index));
        }

        possible_states
    }
}
