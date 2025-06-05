use crate::GameState;
use std::f32;

#[derive(Clone, Debug)]
pub struct Board {
    data: Vec<CellState>,
    width: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Player(u8),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Diagonal {
    Major,
    Minor,
}

#[derive(Debug, Clone)]
pub enum BoardError {
    CellOccupied(usize),
    OutOfBounds(usize),
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::CellOccupied(i) => write!(f, "{i} is not a legal move"),
            BoardError::OutOfBounds(i) => write!(f, "Move {i} would be out of bounds"),
        }
    }
}

const BOARD_STANDARD_WIDTH: usize = 3;

impl Board {
    pub fn new() -> Board {
        Board {
            data: vec![CellState::Empty; BOARD_STANDARD_WIDTH * BOARD_STANDARD_WIDTH],
            width: BOARD_STANDARD_WIDTH,
        }
    }

    /// Creates a new board with given vector as board data. Panics if the resulting board is uneven.
    pub fn new_from(data: Vec<CellState>) -> Board {
        let computed_width = f32::sqrt(data.len() as f32);
        if computed_width % 1.0 != 0.0 {
            panic!("Attempted to make a board of uneven size!");
        }
        Board {
            data,
            width: computed_width.floor() as usize,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_valid_move(&self, index: usize) -> Result<(), BoardError> {
        if index > self.len() - 1 {
            return Err(BoardError::OutOfBounds(index));
        } else if self.data[index] != CellState::Empty {
            return Err(BoardError::CellOccupied(index));
        }
        Ok(())
    }

    pub fn place(&mut self, index: usize, piece_to_place: u8) -> Result<(), BoardError> {
        self.data[index] = CellState::Player(piece_to_place);
        Ok(())
    }

    pub fn render(&self, game: &GameState) {
        println!();
        println!("The board currently looks like this:");
        for row in &self.get_all_rows() {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    CellState::Empty => print!(" "),
                    CellState::Player(piece) => {
                        for p in game.players().iter() {
                            if *piece == p.encoded {
                                print!("{}", p.name);
                            }
                        }
                    }
                }
                if i < self.width - 1 {
                    print!(" | ");
                }
            }
            println!();
            println!("__|___|___");
        }
        println!();
    }

    pub fn render_help(&self) {
        println!();
        println!("This is how you designate the board cells:");
        for (row_index, row) in self.get_all_rows().iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                print!("{}", (row_index * self.width + col_index));
                if col_index < self.width - 1 {
                    print!(" | ");
                }
            }
            println!();
            println!("__|___|___");
        }
        println!();
    }

    pub fn is_full(&self) -> bool {
        self.data.iter().all(|c| *c != CellState::Empty)
    }

    pub fn modify_at_cell(&mut self, pos: usize, new_value: CellState) {
        self.data[pos] = new_value;
    }

    pub fn get_positions_of_empty_cells(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, cell)| **cell == CellState::Empty)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn get_row(&self, row_num: usize) -> Vec<CellState> {
        let start = row_num * self.width;
        let end = start + self.width;
        if start > self.data.len() || end > self.data.len() {
            return vec![];
        }
        self.data[start..end].to_vec()
    }

    pub fn get_all_rows(&self) -> Vec<Vec<CellState>> {
        (0..self.width).map(|n| self.get_row(n)).collect()
    }

    fn get_column(&self, col_num: usize) -> Vec<CellState> {
        if col_num > self.width - 1 {
            return vec![];
        }
        self.data
            .iter()
            .skip(col_num)
            .step_by(self.width)
            .copied()
            .collect()
    }

    pub fn get_all_columns(&self) -> Vec<Vec<CellState>> {
        (0..self.width).map(|n| self.get_column(n)).collect()
    }

    pub fn get_diagonal(&self, diagonal: Diagonal) -> Vec<CellState> {
        let diagonal_num: usize = match diagonal {
            Diagonal::Major => 0,
            Diagonal::Minor => 1,
        };
        let board_size_coefficient = self.width - 1;
        let step = (self.width + 1) / std::cmp::max(1, board_size_coefficient * diagonal_num);
        self.data
            .iter()
            .skip(diagonal_num * board_size_coefficient)
            .step_by(step)
            .take(self.width)
            .copied()
            .collect()
    }

    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self) -> Option<CellState> {
        self.has_horizontal_victor()
            .or_else(|| self.has_vertical_victor())
            .or_else(|| self.has_diagonal_victor())
    }

    fn has_horizontal_victor(&self) -> Option<CellState> {
        for h in self.get_all_rows() {
            let has_victor = self.has_victor(&h);
            if has_victor.is_some() {
                return has_victor;
            }
        }
        None
    }

    fn has_vertical_victor(&self) -> Option<CellState> {
        for v in self.get_all_columns() {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self) -> Option<CellState> {
        self.has_victor(&self.get_diagonal(Diagonal::Major))
            .or_else(|| self.has_victor(&self.get_diagonal(Diagonal::Minor)))
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
                CellState::Empty => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_row {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_row(0),
                vec![CellState::Empty, CellState::Player(1), CellState::Player(2)]
            );
        }

        #[test]
        fn second() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_row(1),
                vec![
                    CellState::Player(3),
                    CellState::Player(4),
                    CellState::Player(5)
                ]
            );
        }

        #[test]
        fn third() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_row(2),
                vec![
                    CellState::Player(6),
                    CellState::Player(7),
                    CellState::Player(8)
                ]
            );
        }

        #[test]
        fn no_such_row() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_row(43), vec![]);
        }
    }

    mod get_column {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_column(0),
                vec![CellState::Empty, CellState::Player(3), CellState::Player(6)]
            );
        }

        #[test]
        fn second() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_column(1),
                vec![
                    CellState::Player(1),
                    CellState::Player(4),
                    CellState::Player(7)
                ]
            );
        }

        #[test]
        fn third() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_column(2),
                vec![
                    CellState::Player(2),
                    CellState::Player(5),
                    CellState::Player(8)
                ]
            );
        }

        #[test]
        fn empty() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_column(3), vec![]);
        }
    }

    mod get_diagonal {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_diagonal(Diagonal::Major),
                vec![CellState::Empty, CellState::Player(4), CellState::Player(8)]
            );
        }

        #[test]
        fn second() {
            let data = vec![
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(3),
                CellState::Player(4),
                CellState::Player(5),
                CellState::Player(6),
                CellState::Player(7),
                CellState::Player(8),
            ];
            let b = Board { data, width: 3 };

            assert_eq!(
                b.get_diagonal(Diagonal::Minor),
                vec![
                    CellState::Player(2),
                    CellState::Player(4),
                    CellState::Player(6)
                ]
            );
        }
    }

    mod new_from {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
            ];

            let board = Board::new_from(data.clone());
            assert_eq!(&board.data, &data);
        }

        #[test]
        #[should_panic]
        fn should_panic() {
            let data = vec![
                CellState::Empty,
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(1),
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(2),
                CellState::Player(2),
            ];
            let _board = Board::new_from(data);
        }
    }
}
