use crate::board::{Board, CellState, Diagonal};

pub struct LogicController {}

impl LogicController {
    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self, board: &Board) -> Option<CellState> {
        self.has_horizontal_victor(board)
            .or_else(|| self.has_vertical_victor(board))
            .or_else(|| self.has_diagonal_victor(board))
    }

    fn has_horizontal_victor(&self, board: &Board) -> Option<CellState> {
        for h in board.get_all_rows() {
            let has_victor = self.has_victor(&h);
            if has_victor.is_some() {
                return has_victor;
            }
        }
        None
    }

    fn has_vertical_victor(&self, board: &Board) -> Option<CellState> {
        for v in board.get_all_columns() {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self, board: &Board) -> Option<CellState> {
        self.has_victor(&board.get_diagonal(Diagonal::Major))
            .or_else(|| self.has_victor(&board.get_diagonal(Diagonal::Minor)))
    }

    fn has_victor(&self, vec: &[CellState]) -> Option<CellState> {
        let first = vec.first();
        first?;
        match first {
            Some(t) => match t {
                CellState::Player(p) => {
                    if vec.iter().all(|&x| x == CellState::Player(*p)) {
                        Some(*first.unwrap())
                    } else {
                        None
                    }
                }
                CellState::AICellValue(_) => None,
                CellState::Empty => None,
            },
            None => None,
        }
    }
}
