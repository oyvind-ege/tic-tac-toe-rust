use std::cmp;
use std::fmt::Display;

use crate::board::*;
use crate::controller::*;
use crate::player::playerlist::*;
use crate::GameState;

pub struct AIMinimax {}

impl PlayerController for AIMinimax {
    fn handle_input(&self, gamestate: &GameState) -> Result<InputType, InputError> {
        let players_info = gamestate.players().get_players_piece_info();
        let best_move = self.find_best_move(gamestate.board(), &players_info);
        println!("Best move is: {best_move}");
        Ok(InputType::Coord(best_move))
    }
}

impl AIMinimax {
    pub fn new() -> AIMinimax {
        AIMinimax {}
    }

    fn find_best_move(&self, board: &Board, players_info: &PlayersInfo) -> usize {
        let possible_moves = board.get_positions_of_empty_cells();
        let mut best_move = possible_moves[0];
        let mut best_score = i8::MIN;

        let mut temporary_board = board.clone();

        for &move_pos in &possible_moves {
            temporary_board.modify_at_cell(move_pos, CellState::Player(players_info.ai_piece));

            let score = self.minimax(&temporary_board, players_info, 0, false);

            if score > best_score {
                best_score = score;
                best_move = move_pos;
            }
            // Resetting the board so we don't have to clone multiple times.
            temporary_board.modify_at_cell(move_pos, CellState::Empty);
        }

        best_move
    }

    fn minimax(
        &self,
        board: &Board,
        players_info: &PlayersInfo,
        depth: i8,
        is_maximizer: bool,
    ) -> i8 {
        const DEFAULT_NEGATIVE_SCORE: i8 = i8::MIN;
        const DEFAULT_POSITIVE_SCORE: i8 = i8::MAX;
        const WINNING_MOVE_SCORE: i8 = 10;
        const LOSING_MOVE_SCORE: i8 = -10;
        const DRAW_MOVE_SCORE: i8 = 0;

        let victor = board.check_for_victory();
        if victor.is_some() || board.is_full() {
            if let Some(winning_piece) = victor {
                if winning_piece == CellState::Player(players_info.ai_piece) {
                    return WINNING_MOVE_SCORE - depth;
                } else {
                    return LOSING_MOVE_SCORE + depth;
                }
            }
            return DRAW_MOVE_SCORE;
        }

        if is_maximizer {
            let mut best = DEFAULT_NEGATIVE_SCORE;
            let possible_state_list = self
                .get_possible_states_from_state(board, CellState::Player(players_info.ai_piece));
            for (state, _) in possible_state_list {
                let new_best = self.minimax(&state, players_info, depth + 1, false);

                best = cmp::max(best, new_best);
            }
            best
        } else {
            let mut best = DEFAULT_POSITIVE_SCORE;
            let possible_state_list = self.get_possible_states_from_state(
                board,
                CellState::Player(players_info.player_piece),
            );

            for (state, _) in possible_state_list {
                best = cmp::min(best, self.minimax(&state, players_info, depth + 1, true))
            }
            best
        }
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
