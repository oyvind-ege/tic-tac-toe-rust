use crate::board::{Board, EMPTY_CELL_SYMBOL};

pub struct LogicController {}

impl LogicController {
    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self, board: &Board) -> Option<u8> {
        self.has_horizontal_victor(board)
            .or_else(|| self.has_vertical_victor(board))
            .or_else(|| self.has_diagonal_victor(board))
    }

    fn has_horizontal_victor(&self, board: &Board) -> Option<u8> {
        for h in board.get_all_rows() {
            let has_victor = self.has_victor(&h);
            if has_victor.is_some() {
                return has_victor;
            }
        }
        None
    }

    fn has_vertical_victor(&self, board: &Board) -> Option<u8> {
        for v in board.get_all_columns() {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self, board: &Board) -> Option<u8> {
        let diagonal_1 = board.get_diagonal(0);
        let diagonal_2 = board.get_diagonal(1);

        self.has_victor(&diagonal_1)
            .or_else(|| self.has_victor(&diagonal_2))
    }

    fn has_victor(&self, vec: &[u8]) -> Option<u8> {
        let first = vec.first();
        first?;
        if *first.unwrap() == EMPTY_CELL_SYMBOL {
            return None;
        }
        if vec.iter().all(|&x| x == *first.unwrap()) {
            Some(*first.unwrap())
        } else {
            None
        }
    }
}
