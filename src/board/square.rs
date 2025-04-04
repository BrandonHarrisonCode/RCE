use std::convert::From;
use std::fmt;

pub mod rays;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
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

// TODO: Change this into a TryFrom
#[allow(clippy::fallible_impl_from)]
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

impl From<Square> for u8 {
    /// Converts a square to a u8 representation, where 0 is in the bottom right corner and 63 is in the top left
    ///
    /// # Arguments
    ///
    /// * `value` - A number that represents the given square, with h1 being 0 and a8 being 63
    ///
    /// # Examples
    /// ```
    /// let squareA1 = Square::from(Square { rank: 0, file: 0 });
    /// let squareA8 = Square::from(Square { rank: 7, file: 0 });
    /// ```
    fn from(value: Square) -> Self {
        value.rank * 8 + value.file
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> Self {
        u8::from(square) as Self
    }
}

impl From<Square> for u64 {
    fn from(square: Square) -> Self {
        1u64 << u8::from(square)
    }
}

impl Square {
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
    pub const fn u8(self) -> u8 {
        self.rank * 8 + self.file
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
    use super::*;
    use crate::board::bitboard::Bitboard;
    use pretty_assertions::assert_eq;

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
            "Rank mask for h6 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
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
            "Rank mask for a2 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
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
            "Rank mask for a2 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
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
            "File mask for h6 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
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
            "File mask for a2 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
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
            "File mask for a2 is incorrect: \nExpected: {:?}\n Got: {:?}",
            Bitboard::new(correct),
            Bitboard::new(result),
        );
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
    fn test_u8_identity() {
        for i in 0..64 {
            let square = Square::from(i);
            let result = square.u8();
            assert_eq!(result, i);
        }
    }

    #[test]
    fn test_usize_identity() {
        for i in 0..64 {
            let square = Square::from(i);
            let result: usize = square.into();
            assert_eq!(result, i as usize);
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
