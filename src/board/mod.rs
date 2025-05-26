use crate::Game;

pub struct Board {
    data: Vec<u8>,
    width: usize,
}

const BOARD_STANDARD_WIDTH: usize = 3;
const EMPTY_CELL_SYMBOL: u8 = 0;

impl Board {
    pub fn new() -> Board {
        Board {
            data: vec![EMPTY_CELL_SYMBOL; BOARD_STANDARD_WIDTH * BOARD_STANDARD_WIDTH],
            width: BOARD_STANDARD_WIDTH,
        }
    }

    // NOTE: We currently do not check if there is already a piece here. May need to return an Option, or a Result
    pub fn place(&mut self, index: usize, piece_to_place: u8) {
        self.data[index] = piece_to_place;
    }

    pub fn render(&self, game: &Game) {
        println!();
        println!("The board currently looks like this:");
        for row in &self.get_all_rows() {
            for (i, cell) in row.iter().enumerate() {
                match *cell {
                    val if val == game.player_1.encoded => print!("{}", game.player_1.name),
                    val if val == game.player_2.encoded => print!("{}", game.player_2.name),
                    _ => print!(" "),
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

    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self) -> Option<u8> {
        self.has_horizontal_victor()
            .or_else(|| self.has_vertical_victor())
            .or_else(|| self.has_diagonal_victor())
    }

    fn has_horizontal_victor(&self) -> Option<u8> {
        for h in self.get_all_rows() {
            let has_victor = self.has_victor(&h);
            if has_victor.is_some() {
                return has_victor;
            }
        }
        None
    }

    fn has_vertical_victor(&self) -> Option<u8> {
        for v in self.get_all_columns() {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self) -> Option<u8> {
        let diagonal_1 = self.get_diagonal(0);
        let diagonal_2 = self.get_diagonal(1);

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

    pub fn get_adjacents(&self, cell_number: usize) -> Vec<u8> {
        self.data
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                //I disregard using overflowing_add because a u8 can easily hold the maximum number of these calculations.
                *index == cell_number.overflowing_sub(1).0
                    || *index == cell_number + 1
                    || *index == cell_number.overflowing_sub(self.width).0
                    || *index == cell_number + self.width
            })
            .map(|(_, cell)| *cell)
            .collect()
    }

    pub fn get_row(&self, row_num: usize) -> Vec<u8> {
        let start = row_num * self.width;
        let end = start + self.width;
        if start > self.data.len() || end > self.data.len() {
            return vec![];
        }
        self.data[start..end].to_vec()
    }

    fn get_all_rows(&self) -> Vec<Vec<u8>> {
        (0..self.width).map(|n| self.get_row(n)).collect()
    }

    fn get_column(&self, col_num: usize) -> Vec<u8> {
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

    fn get_all_columns(&self) -> Vec<Vec<u8>> {
        (0..self.width).map(|n| self.get_column(n)).collect()
    }

    pub fn get_diagonal(&self, diagonal_num: usize) -> Vec<u8> {
        if diagonal_num > 1 {
            return vec![];
        }
        let board_size_coefficient = self.width - 1;
        let step = 4 / std::cmp::max(1, board_size_coefficient * diagonal_num);
        self.data
            .iter()
            .skip(diagonal_num * board_size_coefficient)
            .step_by(step)
            .take(self.width)
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_adjacents {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_adjacents(4), vec![1, 3, 5, 7]);
        }

        #[test]
        fn second() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_adjacents(0), vec![1, 3]);
        }

        #[test]
        fn third() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_adjacents(1), vec![0, 2, 4]);
        }

        #[test]
        fn no_match() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_adjacents(43), vec![]);
        }
    }

    mod get_row {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_row(0), vec![0, 1, 2]);
        }

        #[test]
        fn second() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_row(1), vec![3, 4, 5]);
        }

        #[test]
        fn third() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_row(2), vec![6, 7, 8]);
        }

        #[test]
        fn no_such_row() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_row(43), vec![]);
        }
    }

    mod get_column {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_column(0), vec![0, 3, 6]);
        }

        #[test]
        fn second() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_column(1), vec![1, 4, 7]);
        }

        #[test]
        fn third() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_column(2), vec![2, 5, 8]);
        }

        #[test]
        fn empty() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_column(3), vec![]);
        }
    }

    mod get_diagonal {
        use super::*;

        #[test]
        fn basic() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(0), vec![0, 4, 8]);
        }

        #[test]
        fn second() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(1), vec![2, 4, 6]);
        }

        #[test]
        fn empty() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(3), vec![]);
        }
    }
}
