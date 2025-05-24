/* use crate::{Board, Coordinate, Game};

pub struct AIController {}

impl AIController {
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

                score += self.analyze_freedom_score(&coordinate, game.player_1.encoded, &ai_board);
                score += self.analyze_adjacency_score(&coordinate, &ai_board);
                score += self.analyze_block_score(&coordinate, &ai_board);
                new_board.place(coordinate, score);
            }
        }
        new_board
    }

    //Freedom score represents the number of potential axes of victory you can gain by claiming a position on the board
    /// Accounts for blocked axes
    fn analyze_freedom_score(&self, coordinate: &Coordinate, opponent: u8, board: &Board) -> u8 {
        let all_vectors = [vec![0, 0, 0], vec![opponent; 3]];
        all_vectors
            .iter()
            .filter(|vec| !self.is_blocked(opponent, vec))
            .count()
            .try_into()
            .unwrap()
    }

    fn analyze_adjacency_score(&self, coordinate: &Coordinate, board: &Board) -> u8 {
        todo!();
    }

    fn analyze_block_score(&self, coordinate: &Coordinate, board: &Board) -> u8 {
        0
    }

    fn is_blocked(&self, opponent: u8, vector: &[u8]) -> bool {
        vector.contains(&opponent)
    }

    /// Finds the number of adjacent player_encoded u8s next to Coordinate, on board
    fn get_number_of_adjacents(
        &self,
        player_encoded: u8,
        coordinate: &Coordinate,
        board: &Board,
    ) -> u8 {
        todo!();
    }
}

#[cfg(test)]
mod ai_tests {
    use super::AIController;
    use crate::Board;
    use crate::Coordinate;

    mod pick_best_move {
        use super::*;

        #[test]
        fn basic() {
            let ai = AIController {};
            let ai_generated_score_board = Board {
                number_of_columns: 3,
                #[rustfmt::skip]
                rows: vec![ vec![4, 0, 0],
                            vec![0, 0, 0],
                            vec![0, 0, 0]],
            };
            let expected = Coordinate { x: 0, y: 0 };
            assert_eq!(ai.pick_best_move(ai_generated_score_board), expected)
        }

        #[test]
        fn also_basic() {
            let ai = AIController {};
            let ai_generated_score_board = Board {
                number_of_columns: 3,
                #[rustfmt::skip]
                rows: vec![ vec![4, 0, 0],
                            vec![0, 0, 0],
                            vec![0, 0, 12]],
            };
            let expected = Coordinate { x: 2, y: 2 };
            assert_eq!(ai.pick_best_move(ai_generated_score_board), expected)
        }
    }

    mod is_blocked_tests {
        use super::*;

        #[test]
        fn is_blocked_test() {
            let ai = AIController {};
            assert!(!ai.is_blocked(8, &[0, 0, 0]));
        }

        #[test]
        fn is_blocked_true() {
            let ai = AIController {};
            assert!(ai.is_blocked(8, &[0, 0, 8]));
        }

        #[test]
        fn is_blocked_not() {
            let ai = AIController {};
            assert!(!ai.is_blocked(8, &[0, 32, 0]));
        }
    }

    mod freedom_score_tests {
        use super::*;

        #[test]
        fn freedom_score() {
            let ai = AIController {};
            let board = Board {
                #[rustfmt::skip]
            rows: vec![ vec![0, 0, 0],
                        vec![32, 0, 0],
                        vec![0, 0, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.analyze_freedom_score(&Coordinate { x: 0, y: 0 }, 32, &board),
                2
            );
        }

        #[test]
        fn freedom_score_ok() {
            let ai = AIController {};
            let board = Board {
                #[rustfmt::skip]
            rows: vec![ vec![0, 0, 0],
                        vec![0, 0, 0],
                        vec![0, 32, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.analyze_freedom_score(&Coordinate { x: 0, y: 0 }, 32, &board),
                3
            );
        }

        #[test]
        fn get_number_of_adjacents_vertical() {
            let ai = AIController {};
            let board = Board {
                rows: vec![vec![0, 0, 0], vec![32, 0, 0], vec![0, 0, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.get_number_of_adjacents(32, &Coordinate { x: 0, y: 0 }, &board),
                1
            );
        }
    }

    mod adjacency_score_tests {

        use super::*;

        #[test]
        fn get_number_of_adjacents_horizontal() {
            let ai = AIController {};
            let board = Board {
                #[rustfmt::skip]
            rows: vec![ vec![0, 0, 0],
                        vec![32, 0, 0],
                        vec![0, 0, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.get_number_of_adjacents(32, &Coordinate { x: 1, y: 1 }, &board),
                1
            );
        }

        #[test]
        fn get_number_of_adjacents_horizontals_two() {
            let ai = AIController {};
            let board = Board {
                #[rustfmt::skip]
            rows: vec![ vec![0, 32, 0],
                        vec![32, 0, 0],
                        vec![0, 0, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.get_number_of_adjacents(32, &Coordinate { x: 1, y: 1 }, &board),
                2
            );
        }

        #[test]
        fn get_number_of_adjacents_horizontals_bidirectional() {
            let ai = AIController {};
            let board = Board {
                #[rustfmt::skip]
            rows: vec![ vec![0, 0, 0],
                        vec![32, 0, 32],
                        vec![0, 0, 0]],
                number_of_columns: 3,
            };
            assert_eq!(
                ai.get_number_of_adjacents(32, &Coordinate { x: 1, y: 1 }, &board),
                2
            );
        }
    }
} */
