use std::cmp;

use crate::board::*;
use crate::controller::*;
use crate::player::base_player::PlayerPiece;
use crate::GameState;

pub struct AIMinimax {}

/// Represents a Board State and the Move that made it possible
struct BoardAfterMove((Board, usize));

impl std::ops::Deref for BoardAfterMove {
    type Target = Board;
    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

impl PlayerController for AIMinimax {
    fn handle_input(&self, gamestate: &GameState) -> Result<InputType, InputError> {
        let best_move = self.find_best_move(gamestate);
        Ok(InputType::Coord(best_move))
    }
}

impl AIMinimax {
    pub fn new() -> AIMinimax {
        AIMinimax {}
    }

    fn find_best_move(&self, game_state: &GameState) -> usize {
        let possible_moves = game_state.board().get_indices_of_empty_cells();
        /*
        NOTE: We are at this point assuming the board is not full, quite simply due to the main game logic. See src/gamestate/mod.rs.
        The alternative is to wrap the return value of this function in an Option. That would require some changes to the core data flow.
        */
        let mut best_move = possible_moves[0];
        let mut best_score = i8::MIN;

        let mut temporary_board = game_state.board().clone();

        for &move_index in &possible_moves {
            temporary_board.modify_at_cell(
                move_index,
                CellState::Player(game_state.players().get_ai_player_piece().unwrap()),
            );

            let score = self.minimax(game_state, &temporary_board, 0, false);

            if score > best_score {
                best_score = score;
                best_move = move_index;
            }
            // Resetting the board so we don't have to clone multiple times.
            temporary_board.modify_at_cell(move_index, CellState::Empty);
        }

        best_move
    }

    fn minimax(
        &self,
        game_state: &GameState,
        board_to_analyze: &Board,
        depth: i8,
        is_maximizer: bool,
    ) -> i8 {
        const WINNING_MOVE_SCORE: i8 = 10;
        const LOSING_MOVE_SCORE: i8 = -10;
        const DRAW_MOVE_SCORE: i8 = 0;

        let ai_player_piece = game_state.players().get_ai_player_piece().unwrap();
        let local_human_player_piece = game_state.players().get_local_human_player_piece();

        let winner = game_state.referee().adjudicate(board_to_analyze);
        if winner.is_some() || board_to_analyze.is_full() {
            if let Some(winning_piece) = winner {
                // We can unwrap because we assume there is an AI player at this point
                if winning_piece == ai_player_piece {
                    return WINNING_MOVE_SCORE - depth;
                } else {
                    return LOSING_MOVE_SCORE + depth;
                }
            }
            return DRAW_MOVE_SCORE;
        }

        if is_maximizer {
            let mut best = i8::MIN;
            let possible_board_states = self.get_possible_board_states_from_current_board(
                board_to_analyze,
                CellState::Player(ai_player_piece),
            );
            for board_state in possible_board_states {
                let new_best = self.minimax(game_state, &board_state, depth + 1, false);

                best = cmp::max(best, new_best);
            }
            best
        } else {
            let mut best = i8::MAX;
            let possible_board_states = self.get_possible_board_states_from_current_board(
                board_to_analyze,
                CellState::Player(local_human_player_piece),
            );

            for board_state in possible_board_states {
                best = cmp::min(
                    best,
                    self.minimax(game_state, &board_state, depth + 1, true),
                )
            }
            best
        }
    }

    fn get_possible_board_states_from_current_board(
        &self,
        current_board: &Board,
        player_to_move: CellState,
    ) -> Vec<BoardAfterMove> {
        let mut possible_board_states: Vec<BoardAfterMove> = vec![];
        let all_possible_moves = current_board.get_indices_of_empty_cells();
        for possible_move in all_possible_moves {
            let mut new_state = current_board.clone();
            new_state.modify_at_cell(possible_move, player_to_move);
            possible_board_states.push(BoardAfterMove((new_state, possible_move)));
        }

        possible_board_states
    }
}
