use std::convert::From;
use std::fmt;

pub mod rays;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}

pub struct Delta {
    rank_delta: i8,
    file_delta: i8,
}

#[derive(Clone, Copy)]
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

impl From<&str> for Square {
    /// Creates a new square from a given algebraic notation
    ///
    /// This function is case sensitive and expects the file to be a lowercase letter and the rank to be a number.
    ///
    /// # Arguments
    ///
    /// * `algebraic_notation` - A string that represents the square in algebraic notation
    ///
    /// # Examples
    /// ```
    /// let squareA1 = Square::from("a1");
    /// let squareD4 = Square::from("d4");
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    fn from(algebraic_notation: &str) -> Self {
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

        Self { rank, file }
    }
}

impl From<String> for Square {
    /// Creates a new square from a given algebraic notation
    ///
    /// This function is case sensitive and expects the file to be a lowercase letter and the rank to be a number.
    ///
    /// # Arguments
    ///
    /// * `algebraic_notation` - A string that represents the square in algebraic notation
    ///
    /// # Examples
    /// ```
    /// let squareA1 = Square::from("a1");
    /// let squareD4 = Square::from("d4");
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    fn from(algebraic_notation: String) -> Self {
        Self::from(algebraic_notation.as_str())
    }
}

impl From<u8> for Square {
    /// Creates a new square from an integer
    ///
    /// # Arguments
    ///
    /// * `value` - A number that represents the given square, with h1 being 0 and a8 being 63
    ///
    /// # Examples
    /// ```
    /// let squareA1 = Square::from(55);
    /// let squareA8 = Square::from(0);
    /// ```
    fn from(value: u8) -> Self {
        let file: u8 = value % 8;
        let rank: u8 = value >> 3;
        Self { rank, file }
    }
}

impl Square {
    /// Returns a vec of squares that are between the start and destination squares
    ///
    /// # Arguments
    ///
    /// * `start` - The starting square
    /// * `dest` - The destination square
    ///
    /// # Examples
    /// ```
    /// let squares: Vec<Square> = Square::new("a1").get_transit_squares(Square::new("h8"));
    /// ```
    pub fn get_transit_squares(self, dest: Self) -> Vec<Self> {
        let mut squares: Vec<Self> = vec![];

        let mut rank = self.rank;
        let mut file = self.file;

        while rank != dest.rank || file != dest.file {
            match rank.cmp(&dest.rank) {
                std::cmp::Ordering::Less => rank += 1,
                std::cmp::Ordering::Greater => rank -= 1,
                std::cmp::Ordering::Equal => {}
            }

            match file.cmp(&dest.file) {
                std::cmp::Ordering::Less => file += 1,
                std::cmp::Ordering::Greater => file -= 1,
                std::cmp::Ordering::Equal => {}
            }

            squares.push(Self { rank, file });
        }

        squares.pop(); // Remove the destination square from the list of transit squares
        squares
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
    pub const fn get_mask(self) -> u64 {
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
    pub const fn get_rank_mask(self) -> u64 {
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
    pub const fn get_file_mask(self) -> u64 {
        0x_01010101_01010101 << self.file
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
    pub fn get_diagonals_mask(self) -> u64 {
        let start = self.u8();
        let mut mask = 0u64;

        // Northwest
        let mut step: i128 = i128::from(start);
        while step < 64 {
            mask |= 1 << step;
            if step % 8 == 0 {
                break;
            }
            step += 7;
        }

        // Northeast
        step = i128::from(start);
        while step < 64 {
            mask |= 1 << step;
            if (step + 1) % 8 == 0 {
                break;
            }
            step += 9;
        }

        // Southeast
        step = i128::from(start);
        while step >= 0 {
            mask |= 1 << step;
            if (step + 1) % 8 == 0 {
                break;
            }
            step -= 7;
        }

        // Southwest
        step = i128::from(start);
        while step >= 0 {
            mask |= 1 << step;
            if step % 8 == 0 {
                break;
            }
            step -= 9;
        }

        mask
    }

    /// Converts a square to it's u8 representation, where 0 is in the bottom right corner and 63 is in the top left
    ///
    /// # Arguments
    ///
    /// * `square` - The square to be converted
    ///
    /// # Examples
    /// ```
    /// let num = Square::new("a1").u8();
    /// ```
    pub fn u8(self) -> u8 {
        (self.rank * 8 + self.file).into()
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
    pub fn get_squares_from_mask(mask: u64) -> Vec<Self> {
        let mut squares: Vec<Self> = vec![];

        for i in 0..64 {
            if mask & (1 << i) != 0 {
                squares.push(Self::from(i));
            }
        }

        squares
    }
}

impl std::ops::Add<Delta> for Square {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn add(self, other: Delta) -> Self {
        Self {
            rank: (i16::from(self.rank) + i16::from(other.rank_delta)) as u8,
            file: (i16::from(self.file) + i16::from(other.file_delta)) as u8,
        }
    }
}

impl std::ops::Add<Direction> for Square {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        let delta = match direction {
            Direction::North => Delta {
                rank_delta: 1,
                file_delta: 0,
            },
            Direction::NorthEast => Delta {
                rank_delta: 1,
                file_delta: 1,
            },
            Direction::East => Delta {
                rank_delta: 0,
                file_delta: 1,
            },
            Direction::SouthEast => Delta {
                rank_delta: -1,
                file_delta: 1,
            },
            Direction::South => Delta {
                rank_delta: -1,
                file_delta: 0,
            },
            Direction::SouthWest => Delta {
                rank_delta: -1,
                file_delta: -1,
            },
            Direction::West => Delta {
                rank_delta: 0,
                file_delta: -1,
            },
            Direction::NorthWest => Delta {
                rank_delta: 1,
                file_delta: -1,
            },
        };

        self + delta
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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::super::utils::*;
    use super::*;
    use pretty_assertions::assert_eq;
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
        let after = before + Direction::North;

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_northeast() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::NorthEast;

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_east() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::East;

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_southeast() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::SouthEast;

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_south() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::South;

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_southwest() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::SouthWest;

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_west() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::West;

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_northwest() {
        let before = Square { rank: 4, file: 4 };
        let after = before + Direction::NorthWest;

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_direction_inverse() {
        let square = Square { rank: 4, file: 4 };

        assert_eq!(square, square + Direction::North + Direction::South);
        assert_eq!(square, square + Direction::East + Direction::West);
        assert_eq!(square, square + Direction::NorthWest + Direction::SouthEast);
        assert_eq!(square, square + Direction::NorthEast + Direction::SouthWest);
    }

    #[test]
    fn test_new_square1() {
        let result = Square::from("d5");
        let correct = Square { rank: 4, file: 3 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square2() {
        let result = Square::from("a1");
        let correct = Square { rank: 0, file: 0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square3() {
        let result = Square::from("a8");
        let correct = Square { rank: 7, file: 0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square4() {
        let result = Square::from("h1");
        let correct = Square { rank: 0, file: 7 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square5() {
        let result = Square::from("h8");
        let correct = Square { rank: 7, file: 7 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_new_square6() {
        let result = Square::from("e3");
        let correct = Square { rank: 2, file: 4 };

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_rank_mask_h6() {
        let start_square = Square::from("h6");
        let result = start_square.get_rank_mask();
        let correct = 0b_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;

        assert_eq!(
            result,
            correct,
            "Rank mask for h6 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_rank_mask_a1() {
        let start_square = Square::from("a1");
        let result = start_square.get_rank_mask();
        let correct = 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

        assert_eq!(
            result,
            correct,
            "Rank mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_rank_mask_b8() {
        let start_square = Square::from("b8");
        let result = start_square.get_rank_mask();
        let correct = 0b_11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        assert_eq!(
            result,
            correct,
            "Rank mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_file_mask_h6() {
        let start_square = Square::from("h6");
        let result = start_square.get_file_mask();
        let correct = 0b_10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

        assert_eq!(
            result,
            correct,
            "File mask for h6 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_file_mask_a1() {
        let start_square = Square::from("a1");
        let result = start_square.get_file_mask();
        let correct = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

        assert_eq!(
            result,
            correct,
            "File mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_file_mask_b8() {
        let start_square = Square::from("b8");
        let result = start_square.get_file_mask();
        let correct = 0b_00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;

        assert_eq!(
            result,
            correct,
            "File mask for a2 is incorrect: \nExpected: {}\n Got: {}",
            debug_bitboard(correct),
            debug_bitboard(result),
        );
    }

    #[test]
    fn test_get_squares_from_mask_file_h6() {
        let start_square = Square::from("h6");
        let result = Square::get_squares_from_mask(start_square.get_file_mask());
        let correct = vec![
            Square::from("h1"),
            Square::from("h2"),
            Square::from("h3"),
            Square::from("h4"),
            Square::from("h5"),
            Square::from("h6"),
            Square::from("h7"),
            Square::from("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_h6() {
        let start_square = Square::from("h6");
        let result = Square::get_squares_from_mask(start_square.get_rank_mask());
        let correct = vec![
            Square::from("a6"),
            Square::from("b6"),
            Square::from("c6"),
            Square::from("d6"),
            Square::from("e6"),
            Square::from("f6"),
            Square::from("g6"),
            Square::from("h6"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_and_file_h6() {
        let start_square = Square::from("h6");
        let result = Square::get_squares_from_mask(
            start_square.get_rank_mask() | start_square.get_file_mask(),
        );
        let correct = vec![
            Square::from("a6"),
            Square::from("b6"),
            Square::from("c6"),
            Square::from("d6"),
            Square::from("e6"),
            Square::from("f6"),
            Square::from("g6"),
            Square::from("h1"),
            Square::from("h2"),
            Square::from("h3"),
            Square::from("h4"),
            Square::from("h5"),
            Square::from("h6"),
            Square::from("h7"),
            Square::from("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_file_a1() {
        let start_square = Square::from("a1");
        let result = Square::get_squares_from_mask(start_square.get_file_mask());
        let correct = vec![
            Square::from("a1"),
            Square::from("a2"),
            Square::from("a3"),
            Square::from("a4"),
            Square::from("a5"),
            Square::from("a6"),
            Square::from("a7"),
            Square::from("a8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_a1() {
        let start_square = Square::from("a1");
        let result = Square::get_squares_from_mask(start_square.get_rank_mask());
        let correct = vec![
            Square::from("a1"),
            Square::from("b1"),
            Square::from("c1"),
            Square::from("d1"),
            Square::from("e1"),
            Square::from("f1"),
            Square::from("g1"),
            Square::from("h1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_squares_from_mask_rank_and_file_a1() {
        let start_square = Square::from("a1");
        let result = Square::get_squares_from_mask(
            start_square.get_rank_mask() | start_square.get_file_mask(),
        );
        let correct = vec![
            Square::from("a1"),
            Square::from("b1"),
            Square::from("c1"),
            Square::from("d1"),
            Square::from("e1"),
            Square::from("f1"),
            Square::from("g1"),
            Square::from("h1"),
            Square::from("a2"),
            Square::from("a3"),
            Square::from("a4"),
            Square::from("a5"),
            Square::from("a6"),
            Square::from("a7"),
            Square::from("a8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_a1() {
        let start_square = Square::from("a1");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("a1"),
            Square::from("b2"),
            Square::from("c3"),
            Square::from("d4"),
            Square::from("e5"),
            Square::from("f6"),
            Square::from("g7"),
            Square::from("h8"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_b1() {
        let start_square = Square::from("b1");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("b1"),
            Square::from("c2"),
            Square::from("d3"),
            Square::from("e4"),
            Square::from("f5"),
            Square::from("g6"),
            Square::from("h7"),
            Square::from("a2"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_e4() {
        let start_square = Square::from("e4");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("e4"),
            Square::from("d5"),
            Square::from("c6"),
            Square::from("b7"),
            Square::from("a8"),
            Square::from("f5"),
            Square::from("g6"),
            Square::from("h7"),
            Square::from("d3"),
            Square::from("c2"),
            Square::from("b1"),
            Square::from("f3"),
            Square::from("g2"),
            Square::from("h1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_d4() {
        let start_square = Square::from("d4");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("d4"),
            Square::from("a1"),
            Square::from("b2"),
            Square::from("c3"),
            Square::from("c5"),
            Square::from("b6"),
            Square::from("a7"),
            Square::from("e5"),
            Square::from("f6"),
            Square::from("g7"),
            Square::from("h8"),
            Square::from("e3"),
            Square::from("f2"),
            Square::from("g1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_g6() {
        let start_square = Square::from("g6");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("g6"),
            Square::from("h7"),
            Square::from("h5"),
            Square::from("f7"),
            Square::from("e8"),
            Square::from("f5"),
            Square::from("e4"),
            Square::from("d3"),
            Square::from("c2"),
            Square::from("b1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_get_diagonals_mask_h6() {
        let start_square = Square::from("h6");

        let result = Square::get_squares_from_mask(start_square.get_diagonals_mask());
        let correct = vec![
            Square::from("h6"),
            Square::from("g7"),
            Square::from("f8"),
            Square::from("g5"),
            Square::from("f4"),
            Square::from("e3"),
            Square::from("d2"),
            Square::from("c1"),
        ];

        let result_set: HashSet<Square> = result.into_iter().collect();
        let correct_set: HashSet<Square> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_u8_a1() {
        let start_square = Square::from("a1");
        let result = start_square.u8();
        let correct = 0;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_h1() {
        let start_square = Square::from("h1");
        let result = start_square.u8();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_a8() {
        let start_square = Square::from("a8");
        let result = start_square.u8();
        let correct = 56;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_h8() {
        let start_square = Square::from("h8");
        let result = start_square.u8();
        let correct = 63;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_c5() {
        let start_square = Square::from("c5");
        let result = start_square.u8();
        let correct = 34;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_f3() {
        let start_square = Square::from("f3");
        let result = start_square.u8();
        let correct = 21;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_a1_to_h8() {
        let start_square = Square::from("a1");
        let dest_square = Square::from("h8");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![
            Square::from("b2"),
            Square::from("c3"),
            Square::from("d4"),
            Square::from("e5"),
            Square::from("f6"),
            Square::from("g7"),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_h8_to_a1() {
        let start_square = Square::from("h8");
        let dest_square = Square::from("a1");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![
            Square::from("g7"),
            Square::from("f6"),
            Square::from("e5"),
            Square::from("d4"),
            Square::from("c3"),
            Square::from("b2"),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_a8_to_h1() {
        let start_square = Square::from("a8");
        let dest_square = Square::from("h1");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![
            Square::from("b7"),
            Square::from("c6"),
            Square::from("d5"),
            Square::from("e4"),
            Square::from("f3"),
            Square::from("g2"),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_h1_to_a8() {
        let start_square = Square::from("h1");
        let dest_square = Square::from("a8");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![
            Square::from("g2"),
            Square::from("f3"),
            Square::from("e4"),
            Square::from("d5"),
            Square::from("c6"),
            Square::from("b7"),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_e4_to_e7() {
        let start_square = Square::from("e4");
        let dest_square = Square::from("e7");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![Square::from("e5"), Square::from("e6")];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_e7_to_e4() {
        let start_square = Square::from("e7");
        let dest_square = Square::from("e4");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![Square::from("e6"), Square::from("e5")];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_d3_to_f3() {
        let start_square = Square::from("d3");
        let dest_square = Square::from("f3");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![Square::from("e3")];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_transit_squares_f3_to_d3() {
        let start_square = Square::from("f3");
        let dest_square = Square::from("d3");
        let result = start_square.get_transit_squares(dest_square);
        let correct = vec![Square::from("e3")];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_u8_identity() {
        for i in 0..64 {
            let square = Square::from(i);
            let result = square.u8();
            assert_eq!(result, i);
        }
    }

    #[test]
    fn test_from() {
        for file in (b'a'..=b'h').map(char::from) {
            for rank in 1..=8 {
                let square = Square::from(format!("{}{}", file, rank));
                let num = square.u8();
                assert_eq!(square, Square::from(num));
            }
        }
    }
}
