use crate::GameState;
use std::{cell::Cell, f32};

#[derive(Clone, Debug)]
pub struct Board {
    data: Vec<CellState>,
    width: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Player(u8),
    /// Represents an AI scoring value for this particular cell
    AICellValue(u8),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Diagonal {
    Major,
    /// Also known as the antidiagonal
    Minor,
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

    // NOTE: We currently do not check if there is already a piece here. May need to return an Option, or a Result
    pub fn place(&mut self, index: usize, piece_to_place: u8) {
        self.data[index] = CellState::Player(piece_to_place);
    }

    pub fn render(&self, game: &GameState) {
        println!();
        println!("The board currently looks like this:");
        for row in &self.get_all_rows() {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    CellState::Empty => print!(" "),
                    CellState::Player(piece) => {
                        for p in game.players.iter() {
                            if *piece == p.encoded {
                                print!("{}", p.name);
                            }
                        }
                    }
                    CellState::AICellValue(val) => print!("{val}"),
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

    pub fn get_adjacent_cells(&self, cell_index: usize) -> Vec<CellState> {
        self.data
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                //I disregard using overflowing_add because a u8 can easily hold the maximum number of these calculations.
                *index == cell_index.overflowing_sub(1).0
                    || *index == cell_index + 1
                    || *index == cell_index.overflowing_sub(self.width).0
                    || *index == cell_index + self.width
            })
            .map(|(_, cell)| *cell)
            .collect()
    }

    pub fn get_relevant_axes_of_cell_index(&self, cell_index: usize) -> Vec<Vec<CellState>> {
        let row_number_of_cell = (cell_index as f32 / self.width as f32).floor();
        let row = self.get_row(row_number_of_cell as usize);
        let column_number_of_cell = 0;
        let column = self.get_column(column_number_of_cell);

        let diagonals = self.get_diagonals_of_cell(cell_index).unwrap_or_default();

        vec![row, column, diagonals.into_iter().flatten().collect()]
            .into_iter()
            .filter(|vec| !vec.is_empty())
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

    pub fn get_diagonals_of_cell(&self, cell_index: usize) -> Option<Vec<Vec<CellState>>> {
        // All diagonal cells will have a modulo of 0 with the width of the board - 1
        if cell_index % self.width.overflowing_sub(1).0 != 0 {
            return None;
        }
        let row = (cell_index as f32 / self.width as f32).floor();
        let col = cell_index as f32 % self.width as f32;

        let mut diagonal_data: Vec<Vec<CellState>> = vec![];
        if row == col {
            diagonal_data.push(self.get_diagonal(Diagonal::Major))
        } else if row + col == (self.width - 1) as f32 {
            diagonal_data.push(self.get_diagonal(Diagonal::Minor));
        }

        if diagonal_data.is_empty() {
            None
        } else {
            Some(diagonal_data)
        }
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
                CellState::AICellValue(_) => None,
                CellState::Empty => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_adjacents {
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
                b.get_adjacent_cells(4),
                vec![
                    CellState::Player(1),
                    CellState::Player(3),
                    CellState::Player(5),
                    CellState::Player(7)
                ]
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
                b.get_adjacent_cells(0),
                vec![CellState::Player(1), CellState::Player(3)]
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
                b.get_adjacent_cells(1),
                vec![CellState::Empty, CellState::Player(2), CellState::Player(4)]
            );
        }

        #[test]
        fn no_match() {
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

            assert_eq!(b.get_adjacent_cells(43), vec![]);
        }
    }

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

    mod get_relevant_axes_of_cell {
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

            let board = Board::new_from(data);
            assert_eq!(
                board.get_relevant_axes_of_cell_index(0),
                vec![
                    vec![CellState::Empty, CellState::Player(1), CellState::Player(2)],
                    vec![CellState::Empty, CellState::Player(3), CellState::Player(6)],
                    vec![CellState::Empty, CellState::Player(4), CellState::Player(8)]
                ]
            );
        }

        #[test]
        fn second() {
            let data = vec![
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Player(1),
                CellState::Player(1),
                CellState::Player(1),
                CellState::Player(2),
                CellState::Player(2),
                CellState::Player(2),
            ];

            let board = Board::new_from(data);
            assert_eq!(
                board.get_relevant_axes_of_cell_index(1),
                vec![
                    vec![CellState::Empty, CellState::Empty, CellState::Empty],
                    vec![CellState::Empty, CellState::Player(1), CellState::Player(2)]
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

            let board = Board::new_from(data);
            assert_eq!(
                board.get_relevant_axes_of_cell_index(4),
                vec![
                    vec![
                        CellState::Player(3),
                        CellState::Player(4),
                        CellState::Player(5)
                    ],
                    vec![
                        CellState::Player(1),
                        CellState::Player(4),
                        CellState::Player(7)
                    ],
                    vec![
                        CellState::Player(2),
                        CellState::Player(4),
                        CellState::Player(6)
                    ],
                    vec![CellState::Empty, CellState::Player(4), CellState::Player(8)]
                ]
            );
        }

        #[test]
        fn fourth() {
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

            let board = Board::new_from(data);
            assert_eq!(
                board.get_relevant_axes_of_cell_index(8),
                vec![
                    vec![
                        CellState::Player(2),
                        CellState::Player(5),
                        CellState::Player(8)
                    ],
                    vec![
                        CellState::Player(6),
                        CellState::Player(7),
                        CellState::Player(8)
                    ],
                    vec![CellState::Empty, CellState::Player(4), CellState::Player(8)]
                ]
            );
        }
    }

    mod get_diagonals_of_cell {
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

            let board = Board::new_from(data);
            assert_eq!(
                board.get_diagonals_of_cell(0),
                Some(vec![vec![
                    CellState::Empty,
                    CellState::Player(4),
                    CellState::Player(8)
                ]])
            )
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

            let board = Board::new_from(data);
            assert_eq!(
                board.get_diagonals_of_cell(4),
                Some(vec![
                    vec![CellState::Empty, CellState::Player(4), CellState::Player(8)],
                    vec![
                        CellState::Player(2),
                        CellState::Player(4),
                        CellState::Player(6)
                    ]
                ])
            )
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

            let board = Board::new_from(data);
            assert_eq!(board.get_diagonals_of_cell(1), None)
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
