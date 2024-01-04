use std::fmt;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}
impl Square {
    #[allow(dead_code)]
    pub fn new(algebraic_notation: &str) -> Square {
        let mut iter = algebraic_notation.chars();
        let filechar: char = iter.next().unwrap();

        let file: u8 = filechar as u8 - b'a';
        let rank: u8 = (iter
            .next()
            .unwrap()
            .to_string()
            .parse::<u16>()
            .ok()
            .unwrap()
            - 1) as u8;

        Square { rank, file }
    }

    /// Returns a u64 mask filled with 0s except for a 1 in the designated square
    ///
    /// # Arguments
    ///
    /// * `square` - A square that indicates the desired bit to set to 1
    ///
    /// # Examples
    /// ```
    /// let mask = Square::new("e2").get_mask();
    /// ```
    pub fn get_mask(&self) -> u64 {
        self.get_rank_mask() & self.get_file_mask()
    }

    /// Creates a mask that marks the rank of a given square
    ///
    /// # Arguments
    ///
    /// * `square` - The square that will be covered by the mask
    ///
    /// # Examples
    /// ```
    /// let rank_mask = Square::new("a1").get_rank_mask());
    /// ```
    pub fn get_rank_mask(&self) -> u64 {
        0xFF << (self.rank * 8)
    }

    /// Creates a mask that marks the file of a given square
    ///
    /// # Arguments
    ///
    /// * `square` - The square that will be covered by the mask
    ///
    /// # Examples
    /// ```
    /// let file_mask = Square::new("a1").get_file_mask();
    /// ```
    pub fn get_file_mask(&self) -> u64 {
        0x0101010101010101 << (8 - (self.file + 1))
    }

    /// Creates a mask that marks both diagonals of a given square
    ///
    /// # Arguments
    ///
    /// * `square` - The square that will be covered by the mask
    ///
    /// # Examples
    /// ```
    /// let diagonals_mask = Square::new("a1").get_diagonals_mask();
    /// ```
    pub fn get_diagonals_mask(&self) -> u64 {
        let start = self.u64();
        let mut mask = 0u64;

        let mut step: i128 = start as i128;
        while step < 64 {
            mask |= 1 << step;
            if step % 8 == 0 {
                break;
            }
            step += 7;
        }

        step = start as i128;
        while step < 64 {
            mask |= 1 << step;
            if (step + 1) % 8 == 0 {
                break;
            }
            step += 9;
        }

        step = start as i128;
        while step >= 0 {
            mask |= 1 << step;
            if (step + 1) % 8 == 0 {
                break;
            }
            step -= 7;
        }

        step = start as i128;
        while step >= 0 {
            mask |= 1 << step;
            if step % 8 == 0 {
                break;
            }
            step -= 9;
        }

        mask
    }

    /// Converts a square to it's u64 representation, where 0 is in the bottom right corner and 63 is in the top left
    ///
    /// # Arguments
    ///
    /// * `square` - The square to be converted
    ///
    /// # Examples
    /// ```
    /// let num = Square::new("a1").u64();
    /// ```
    fn u64(&self) -> u64 {
        (self.rank * 8 + (7 - self.file)).into()
    }

    /// Returns a vector of squares that match a given mask
    ///
    /// # Arguments
    ///
    /// * `mask` - The mask of squares to be returned
    ///
    /// # Examples
    /// ```
    /// let squares = Square::get_squares_from_mask(0xFF);
    /// ```
    pub fn get_squares_from_mask(mask: u64) -> Vec<Square> {
        let mut squares: Vec<Square> = vec![];

        for i in 0..64 {
            if mask & (1 << i) != 0 {
                squares.push(Square {
                    rank: i / 8,
                    file: 7 - (i % 8),
                });
            }
        }

        squares
    }
}
impl std::ops::Add<SquareDelta> for Square {
    type Output = Square;

    fn add(self, other: SquareDelta) -> Square {
        Square {
            rank: (self.rank as i16 + other.rank_delta as i16) as u8,
            file: (self.file as i16 + other.file_delta as i16) as u8,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.file >= 8 || self.rank >= 8 {
            write!(f, "Invalid range: {}, {}", self.rank, self.file)
        } else {
            let filechar: char = (97 + self.file) as char;
            write!(f, "{}{}", filechar, self.rank + 1)
        }
    }
}

pub struct SquareDelta {
    rank_delta: i8,
    file_delta: i8,
}

#[allow(dead_code)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn unit_square(self) -> SquareDelta {
        match self {
            Self::North => SquareDelta {
                rank_delta: 1,
                file_delta: 0,
            },
            Self::NorthEast => SquareDelta {
                rank_delta: 1,
                file_delta: 1,
            },
            Self::East => SquareDelta {
                rank_delta: 0,
                file_delta: 1,
            },
            Self::SouthEast => SquareDelta {
                rank_delta: -1,
                file_delta: 1,
            },
            Self::South => SquareDelta {
                rank_delta: -1,
                file_delta: 0,
            },
            Self::SouthWest => SquareDelta {
                rank_delta: -1,
                file_delta: -1,
            },
            Self::West => SquareDelta {
                rank_delta: 0,
                file_delta: -1,
            },
            Self::NorthWest => SquareDelta {
                rank_delta: 1,
                file_delta: -1,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::super::utils::*;
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_derived_traits() {
        let square = Square { rank: 3, file: 5 };
        dbg!(&square);

        assert_eq!(square, square.clone());
    }

    #[test]
    fn test_display() {
        let square = Square { rank: 3, file: 5 };

        let result = square.to_string();
        let correct = String::from("f4");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_rank() {
        let square = Square { rank: 9, file: 5 };

        let result = square.to_string();
        let correct = String::from("Invalid range: 9, 5");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_file() {
        let square = Square { rank: 5, file: 9 };

        let result = square.to_string();
        let correct = String::from("Invalid range: 5, 9");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_both() {
        let square = Square { rank: 10, file: 19 };

        let result = square.to_string();
        let correct = String::from("Invalid range: 10, 19");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_north() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::North.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_northeast() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::NorthEast.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_east() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::East.unit_square();

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_southeast() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::SouthEast.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_south() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::South.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_southwest() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::SouthWest.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_west() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::West.unit_square();

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_northwest() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::NorthWest.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_direction_inverse() {
        let square = Square { rank: 4, file: 4 };

        assert_eq!(
            square,
            square + Direction::North.unit_square() + Direction::South.unit_square()
        );
        assert_eq!(
            square,
            square + Direction::East.unit_square() + Direction::West.unit_square()
        );
        assert_eq!(
            square,
            square + Direction::NorthWest.unit_square() + Direction::SouthEast.unit_square()
        );
        assert_eq!(
            square,
            square + Direction::NorthEast.unit_square() + Direction::SouthWest.unit_square()
        );
    }

    #[test]
    fn test_new_square1() {
        let result = Square::new("d5");
        let correct = Square { rank: 4, file: 3 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square2() {
        let result = Square::new("a1");
        let correct = Square { rank: 0, file: 0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square3() {
        let result = Square::new("a8");
        let correct = Square { rank: 7, file: 0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square4() {
        let result = Square::new("h1");
        let correct = Square { rank: 0, file: 7 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square5() {
        let result = Square::new("h8");
        let correct = Square { rank: 7, file: 7 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square6() {
        let result = Square::new("e3");
        let correct = Square { rank: 2, file: 4 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_rank_mask_h6() {
        let start_square = Square::new("h6");
        let result = start_square.get_rank_mask();
        let correct = 0b0000000000000000111111110000000000000000000000000000000000000000;

        assert_eq!(
            result,
            correct,
            "Rank mask for h6 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_rank_mask_a1() {
        let start_square = Square::new("a1");
        let result = start_square.get_rank_mask();
        let correct = 0b0000000000000000000000000000000000000000000000000000000011111111;

        assert_eq!(
            result,
            correct,
            "Rank mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_rank_mask_b8() {
        let start_square = Square::new("b8");
        let result = start_square.get_rank_mask();
        let correct = 0b1111111100000000000000000000000000000000000000000000000000000000;

        assert_eq!(
            result,
            correct,
            "Rank mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_file_mask_h6() {
        let start_square = Square::new("h6");
        let result = start_square.get_file_mask();
        let correct = 0b0000000100000001000000010000000100000001000000010000000100000001;

        assert_eq!(
            result,
            correct,
            "File mask for h6 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_file_mask_a1() {
        let start_square = Square::new("a1");
        let result = start_square.get_file_mask();
        let correct = 0b1000000010000000100000001000000010000000100000001000000010000000;

        assert_eq!(
            result,
            correct,
            "File mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_file_mask_b8() {
        let start_square = Square::new("b8");
        let result = start_square.get_file_mask();
        let correct = 0b0100000001000000010000000100000001000000010000000100000001000000;

        assert_eq!(
            result,
            correct,
            "File mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(&correct),
            debug_bitboard(&result),
        );
    }

    #[test]
    fn test_get_squares_from_mask_file_h6() {
        let start_square = Square::new("h6");
        let result = Square::get_squares_from_mask(start_square.get_file_mask());
        let correct = vec![
            Square::new("h1"),
            Square::new("h2"),
            Square::new("h3"),
            Square::new("h4"),
            Square::new("h5"),
            Square::new("h6"),
            Square::new("h7"),
            Square::new("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_h6() {
        let start_square = Square::new("h6");
        let result = Square::get_squares_from_mask(start_square.get_rank_mask());
        let correct = vec![
            Square::new("a6"),
            Square::new("b6"),
            Square::new("c6"),
            Square::new("d6"),
            Square::new("e6"),
            Square::new("f6"),
            Square::new("g6"),
            Square::new("h6"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_and_file_h6() {
        let start_square = Square::new("h6");
        let result = Square::get_squares_from_mask(
            start_square.get_rank_mask() | start_square.get_file_mask(),
        );
        let correct = vec![
            Square::new("a6"),
            Square::new("b6"),
            Square::new("c6"),
            Square::new("d6"),
            Square::new("e6"),
            Square::new("f6"),
            Square::new("g6"),
            Square::new("h1"),
            Square::new("h2"),
            Square::new("h3"),
            Square::new("h4"),
            Square::new("h5"),
            Square::new("h6"),
            Square::new("h7"),
            Square::new("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_file_a1() {
        let start_square = Square::new("a1");
        let result = Square::get_squares_from_mask(start_square.get_file_mask());
        let correct = vec![
            Square::new("a1"),
            Square::new("a2"),
            Square::new("a3"),
            Square::new("a4"),
            Square::new("a5"),
            Square::new("a6"),
            Square::new("a7"),
            Square::new("a8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_a1() {
        let start_square = Square::new("a1");
        let result = Square::get_squares_from_mask(start_square.get_rank_mask());
        let correct = vec![
            Square::new("a1"),
            Square::new("b1"),
            Square::new("c1"),
            Square::new("d1"),
            Square::new("e1"),
            Square::new("f1"),
            Square::new("g1"),
            Square::new("h1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_and_file_a1() {
        let start_square = Square::new("a1");
        let result = Square::get_squares_from_mask(
            start_square.get_rank_mask() | start_square.get_file_mask(),
        );
        let correct = vec![
            Square::new("a1"),
            Square::new("b1"),
            Square::new("c1"),
            Square::new("d1"),
            Square::new("e1"),
            Square::new("f1"),
            Square::new("g1"),
            Square::new("h1"),
            Square::new("a2"),
            Square::new("a3"),
            Square::new("a4"),
            Square::new("a5"),
            Square::new("a6"),
            Square::new("a7"),
            Square::new("a8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_a1() {
        let start_square = Square::new("a1");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("a1"),
            Square::new("b2"),
            Square::new("c3"),
            Square::new("d4"),
            Square::new("e5"),
            Square::new("f6"),
            Square::new("g7"),
            Square::new("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_b1() {
        let start_square = Square::new("b1");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("b1"),
            Square::new("c2"),
            Square::new("d3"),
            Square::new("e4"),
            Square::new("f5"),
            Square::new("g6"),
            Square::new("h7"),
            Square::new("a2"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_e4() {
        let start_square = Square::new("e4");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("e4"),
            Square::new("d5"),
            Square::new("c6"),
            Square::new("b7"),
            Square::new("a8"),
            Square::new("f5"),
            Square::new("g6"),
            Square::new("h7"),
            Square::new("d3"),
            Square::new("c2"),
            Square::new("b1"),
            Square::new("f3"),
            Square::new("g2"),
            Square::new("h1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_d4() {
        let start_square = Square::new("d4");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("d4"),
            Square::new("a1"),
            Square::new("b2"),
            Square::new("c3"),
            Square::new("c5"),
            Square::new("b6"),
            Square::new("a7"),
            Square::new("e5"),
            Square::new("f6"),
            Square::new("g7"),
            Square::new("h8"),
            Square::new("e3"),
            Square::new("f2"),
            Square::new("g1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_g6() {
        let start_square = Square::new("g6");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("g6"),
            Square::new("h7"),
            Square::new("h5"),
            Square::new("f7"),
            Square::new("e8"),
            Square::new("f5"),
            Square::new("e4"),
            Square::new("d3"),
            Square::new("c2"),
            Square::new("b1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_h6() {
        let start_square = Square::new("h6");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::new("h6"),
            Square::new("g7"),
            Square::new("f8"),
            Square::new("g5"),
            Square::new("f4"),
            Square::new("e3"),
            Square::new("d2"),
            Square::new("c1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_u64_a1() {
        let start_square = Square::new("a1");
        let result = start_square.u64();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u64_h1() {
        let start_square = Square::new("h1");
        let result = start_square.u64();
        let correct = 0;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u64_a8() {
        let start_square = Square::new("a8");
        let result = start_square.u64();
        let correct = 63;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u64_h8() {
        let start_square = Square::new("h8");
        let result = start_square.u64();
        let correct = 56;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u64_c5() {
        let start_square = Square::new("c5");
        let result = start_square.u64();
        let correct = 37;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u64_f3() {
        let start_square = Square::new("f3");
        let result = start_square.u64();
        let correct = 18;

        assert_eq!(result, correct);
    }
}
