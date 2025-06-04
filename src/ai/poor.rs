use crate::board::*;
use crate::controller::*;
use crate::gamestate::*;

pub struct AIPlayer {
    test_board: Board,
    encoded: u8,
}

impl PlayerController for AIPlayer {
    fn handle_input(&self, gamestate: &GameState) -> Result<InputType, InputError> {
        Ok(InputType::Coord(4))
    }
}

impl AIPlayer {
    pub fn new(encoded: u8) -> AIPlayer {
        AIPlayer {
            test_board: Board::new(),
            encoded,
        }
    }

    fn is_blocked(&self, opponent: u8, vector: &[u8]) -> bool {
        vector.contains(&opponent)
    }

    fn generate_ai_move_score_board(&self, board: &Board) -> Board {
        todo!();
    }

    fn calculate_freedom_score(&self, cell: usize) -> u8 {
        0
    }

    fn calculate_adjacency_score(&self, board: &Board, cell_number: usize) -> u8 {
        board
            .get_adjacent_cells(cell_number)
            .iter()
            .filter(|cell| **cell == CellState::Player(self.encoded))
            .count()
            .try_into()
            .unwrap()
    }

    fn calculate_blocked_score(&self, cell: usize) -> u8 {
        0
    }

    fn calculate_win_score(&self, cell: usize) -> u8 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::AIPlayer;
    use crate::board::Board;

    mod is_blocked_tests {
        use super::*;

        #[test]
        fn is_blocked_test() {
            let ai = AIPlayer::new(64);
            assert!(!ai.is_blocked(8, &[0, 0, 0]));
        }

        #[test]
        fn is_blocked_true() {
            let ai = AIPlayer::new(64);
            assert!(ai.is_blocked(8, &[0, 0, 8]));
        }

        #[test]
        fn is_blocked_not() {
            let ai = AIPlayer::new(64);
            assert!(!ai.is_blocked(8, &[0, 32, 0]));
        }
    }

    mod adjacency_score_tests {
        use super::*;
        use crate::board::CellState;

        #[test]
        fn basic() {
            let board = Board::new_from(vec![
                CellState::Empty,
                CellState::Player(32),
                CellState::Empty,
                CellState::Player(32),
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Player(32),
                CellState::Empty,
            ]);
            let ai = AIPlayer {
                test_board: Board::new(),
                encoded: 32,
            };

            assert_eq!(ai.calculate_adjacency_score(&board, 4), 3);
        }
    }
}
