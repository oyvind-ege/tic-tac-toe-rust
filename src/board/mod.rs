use crate::Game;

pub struct Board {
    data: Vec<u8>,
    width: usize,
}

const BOARD_STANDARD_SIZE: usize = 3;

impl Board {
    pub fn new() -> Board {
        Board {
            data: vec![0; BOARD_STANDARD_SIZE * BOARD_STANDARD_SIZE],
            width: BOARD_STANDARD_SIZE,
        }
    }

    // NOTE: We currently do not check if there is already a piece here. May need to return an Option, or a Result
    pub fn place(&mut self, index: usize, piece_to_place: u8) {
        self.data[index] = piece_to_place;
    }

    pub fn render(&self, game: &Game) {
        println!();
        println!("The board currently looks like this:");
        for (i, cell) in self.data.iter().enumerate() {
            match *cell {
                val if val == game.player_1.encoded => print!("{}", game.player_1.name),
                val if val == game.player_2.encoded => print!("{}", game.player_2.name),
                val => print!("{val}"),
            }
            if i < 2 || i > 2 && i < 5 || i > 5 && i < 8 {
                print!(" | ");
            }

            if i == 2 || i == 5 || i == 8 {
                println!();
            }
        }
        println!("__|___|___");
        println!();
    }

    pub fn render_help(&self) {
        println!();
        println!("This is how you designate the board cells:");
        for (index, _) in self.data.iter().enumerate() {
            print!("{}", index);
            if index < 2 {
                print!(" | ");
            }
        }
        println!();
        println!("__|___|___");
        println!();
    }

    /// Return the encoded player symbol if they have won, None if no victor
    pub fn check_for_victory(&self) -> Option<u8> {
        self.has_horizontal_victor()
            .or_else(|| self.has_vertical_victor())
            .or_else(|| self.has_diagonal_victor())
    }

    fn has_horizontal_victor(&self) -> Option<u8> {
        for offset in 0..self.width {
            self.has_victor(&self.data[offset..offset + self.width]);
        }
        None
    }

    fn has_vertical_victor(&self) -> Option<u8> {
        //TODO: Rewrite. Can be much simpler I am sure.
        let mut verticals: Vec<Vec<u8>> = vec![vec![], vec![], vec![]];

        for i in 0..self.width {
            verticals.push(self.get_column(i));
        }

        for v in verticals {
            let has_victor = self.has_victor(&v);
            if has_victor.is_some() {
                return has_victor;
            }
        }

        None
    }

    fn has_diagonal_victor(&self) -> Option<u8> {
        let diagonal_1 = self.get_diagonal(1);
        let diagonal_2 = self.get_diagonal(2);

        self.has_victor(&diagonal_1)
            .or_else(|| self.has_victor(&diagonal_2))
    }

    fn has_victor(&self, vec: &[u8]) -> Option<u8> {
        let first = vec.first();
        first?;
        if *first.unwrap() == 0 {
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

    pub fn get_column(&self, col_num: usize) -> Vec<u8> {
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

    pub fn get_diagonal(&self, diagonal_num: usize) -> Vec<u8> {
        if diagonal_num > 1 {
            return vec![];
        }
        let size_coefficient = self.width - 1;
        let step = 4 / std::cmp::max(1, size_coefficient * diagonal_num);
        self.data
            .iter()
            .skip(diagonal_num * size_coefficient)
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
