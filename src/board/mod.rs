pub struct Board {
    data: Vec<u8>,
    width: usize,
}

impl Board {
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
        todo!();
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

            assert_eq!(b.get_diagonal(0), vec![0, 3, 6]);
        }

        #[test]
        fn second() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(1), vec![1, 4, 7]);
        }

        #[test]
        fn third() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(2), vec![2, 5, 8]);
        }

        #[test]
        fn empty() {
            let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let b = Board { data, width: 3 };

            assert_eq!(b.get_diagonal(3), vec![]);
        }
    }
}
