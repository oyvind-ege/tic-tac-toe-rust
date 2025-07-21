use crate::{player::base_player::PlayerPiece, GameState};
use std::{f32, ops::Deref};
use synonym::Synonym;

#[derive(Clone, Debug)]
pub struct Board {
    data: Vec<CellState>,
    width: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Player(PlayerPiece),
}

#[derive(PartialEq, Eq, Clone, Debug, Synonym)]
pub struct BoardRow(Vec<CellState>);

// We add this so that we can write more generic victory checker functions that act on Vec<CellState>
impl Deref for BoardRow {
    type Target = Vec<CellState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Synonym)]
pub struct BoardColumn(Vec<CellState>);

// We add this so that we can write more generic victory checker functions that act on Vec<CellState>
impl Deref for BoardColumn {
    type Target = Vec<CellState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Synonym)]
pub struct BoardDiagonal(Vec<CellState>);

// We add this so that we can write more generic victory checker functions that act on Vec<CellState>
impl Deref for BoardDiagonal {
    type Target = Vec<CellState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

    /// Checks whther a move to a given index  on the board is valid.
    pub fn is_valid_move(&self, index: usize) -> Result<(), BoardError> {
        if index > self.len() - 1 {
            return Err(BoardError::OutOfBounds(index));
        } else if self.data[index] != CellState::Empty {
            return Err(BoardError::CellOccupied(index));
        }
        Ok(())
    }

    /// Directly edits the game board at specified index, and adds the piece.
    // TODO: Do some error checking here, or redo the return value
    pub fn place(&mut self, index: usize, piece_to_place: PlayerPiece) -> Result<(), BoardError> {
        self.data[index] = CellState::Player(piece_to_place);
        Ok(())
    }

    // TODO: Move this responsibility to a separate module/object
    pub fn render(&self, game: &GameState) {
        println!();
        println!("The board currently looks like this:");
        for row in &self.get_all_rows() {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    CellState::Empty => print!(" "),
                    CellState::Player(piece) => {
                        for p in game.players().iter() {
                            if *piece == p.player_piece {
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

    // TODO: This should not be part of the board object, but instead part of `GameState` or somesuch
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

    /// Checks whether the board is full of `Player` pieces
    pub fn is_full(&self) -> bool {
        self.data.iter().all(|c| *c != CellState::Empty)
    }

    // I primarily use the following method in the minimax algorithm, in order to populate the board with a given move, and then revert back to the original board
    /// A very sketchy method that directly mutates the value at cell index.
    /// Use cautiously!
    pub fn modify_at_cell(&mut self, pos: usize, new_value: CellState) {
        self.data[pos] = new_value;
    }

    /// Returns a vector of indices on the board that are empty
    pub fn get_positions_of_empty_cells(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| {
                if cell == CellState::Empty {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get a copy of a row
    ///
    /// * `row_num`: the row number, counted from 0 and from the top
    pub fn get_row(&self, row_num: usize) -> BoardRow {
        let start = row_num * self.width;
        let end = start + self.width;
        if start > self.data.len() || end > self.data.len() {
            return BoardRow(vec![]);
        }
        BoardRow(self.data[start..end].to_vec())
    }

    /// Get all rows, from top to bottom
    pub fn get_all_rows(&self) -> Vec<BoardRow> {
        (0..self.width).map(|n| self.get_row(n)).collect()
    }

    /// Get a copy of a column
    ///
    /// * `col_num`: the index of the column you want, counting from 0 and from the left
    fn get_column(&self, col_num: usize) -> BoardColumn {
        if col_num > self.width - 1 {
            return BoardColumn(vec![]);
        }
        BoardColumn(
            self.data
                .iter()
                .skip(col_num)
                .step_by(self.width)
                .copied()
                .collect(),
        )
    }

    pub fn get_all_columns(&self) -> Vec<BoardColumn> {
        (0..self.width).map(|n| self.get_column(n)).collect()
    }

    /// Gets a copy of given diagonal from the Board.
    ///
    /// A diagonal is either a Major diagonal, or a Minor diagonal (also called antidiagonal).
    /// A major diagonal is from top left to bottom right; a Minor is from top right to bottom left
    pub fn get_diagonal(&self, diagonal: Diagonal) -> BoardDiagonal {
        let diagonal_num: usize = match diagonal {
            Diagonal::Major => 0,
            Diagonal::Minor => 1,
        };
        let board_size_coefficient = self.width - 1;
        let step = (self.width + 1) / std::cmp::max(1, board_size_coefficient * diagonal_num);
        BoardDiagonal(
            self.data
                .iter()
                .skip(diagonal_num * board_size_coefficient)
                .step_by(step)
                .take(self.width)
                .copied()
                .collect(),
        )
    }

    // TODO: Get rid of CellState as returned Option value, since it does not make conceptual sense.
    /// Checks if a player has won along either of the axes.
    /// Relevant axes are: All rows, all columns, all diagonals.
    /// The returned Option contains the CellState, with the winning Player
    pub fn check_for_victory(&self) -> Option<CellState> {
        self.has_horizontal_victor()
            .or_else(|| self.has_vertical_victor())
            .or_else(|| self.has_diagonal_victor())
    }

    // TODO: Get rid of CellState as returned Option value, since it does not make conceptual sense.
    fn has_horizontal_victor(&self) -> Option<CellState> {
        for h in self.get_all_rows() {
            let has_victor = self.has_victor(&h);
            if has_victor.is_some() {
                return has_victor;
            }
        }
        None
    }

    // TODO: Get rid of CellState as returned Option value, since it does not make conceptual sense.
    fn has_vertical_victor(&self) -> Option<CellState> {
        for v in self.get_all_columns() {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    // TODO: Get rid of CellState as returned Option value, since it does not make conceptual sense.
    fn has_diagonal_victor(&self) -> Option<CellState> {
        self.has_victor(&self.get_diagonal(Diagonal::Major))
            .or_else(|| self.has_victor(&self.get_diagonal(Diagonal::Minor)))
    }

    // TODO: Get rid of CellState as returned Option value, since it does not make conceptual sense.
    /// Checks to see if all CellStates in `vec` contain a Player piece. If so, returns Some(player_piece)
    fn has_victor(&self, vec: &[CellState]) -> Option<CellState> {
        // Just checking to see if the symbol on the first cell is the same as all symbols in all cells
        if let Some(first_cell) = vec.first() {
            if let CellState::Player(player_piece) = first_cell {
                if vec
                    .iter()
                    .all(|&board_cell| board_cell == CellState::Player(*player_piece))
                {
                    return Some(*first_cell);
                } else {
                    return None;
                }
            }
        }
        None
    }
}
