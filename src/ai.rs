use crate::board::CellState;
use crate::InputType;
use crate::{Board, GameState, PlayerController};

pub struct AIPlayer {
    test_board: Board,
    encoded: u8,
}

impl PlayerController for AIPlayer {
    fn handle_input(&self, gamestate: &GameState) -> Option<InputType> {
        Some(InputType::Coord(4))
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

/*

impl AIPlayer {
    pub fn do_move(&self, game: &Game) -> Coordinate {
        self.pick_best_move(self.generate_ai_move_score_board(game))
    }

    fn pick_best_move(&self, ai_board: Board) -> Coordinate {
        let (max_index, max) = ai_board
            .rows
            .iter()
            .flatten()
            .enumerate()
            .max_by_key(|(_i, val)| *val)
            .unwrap();
        ai_board.get_coordinate_from_board_index(max_index.try_into().unwrap())
    }

    fn generate_ai_move_score_board(&self, game: &Game) -> Board {
        let ai_board = game.board.clone();
        let mut new_board = Board::new();
        for (y, row) in ai_board.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let coordinate = Coordinate { x, y };
                let mut score: u8 = 0;
                if *cell == game.player_1.encoded || *cell == game.player_2.encoded {
                    new_board.place(coordinate, score);
                    break;
                }

                score += self.calculate_freedom_score(&coordinate, game.player_1.encoded, &ai_board);
                score += self.calculate_adjacency_score(&coordinate, &ai_board);
                score += self.calculate_block_score(&coordinate, &ai_board);
                new_board.place(coordinate, score);
            }
        }
        new_board
    }

    //Freedom score represents the number of potential axes of victory you can gain by claiming a position on the board
    /// Accounts for blocked axes
    fn calculate_freedom_score(&self, coordinate: &Coordinate, opponent: u8, board: &Board) -> u8 {
        let all_vectors = [vec![0, 0, 0], vec![opponent; 3]];
        all_vectors
            .iter()
            .filter(|vec| !self.is_blocked(opponent, vec))
            .count()
            .try_into()
            .unwrap()
    }

    fn calculate_adjacency_score(&self, coordinate: &Coordinate, board: &Board) -> u8 {
        todo!();
    }

    fn calculate_block_score(&self, coordinate: &Coordinate, board: &Board) -> u8 {
        0
    }


}
*/

#[cfg(test)]
mod tests {
    use super::AIPlayer;
    use crate::Board;

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

    mod freedom_score_tests {
        use super::*;
    }

    mod adjacency_score_tests {
        use super::*;

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
