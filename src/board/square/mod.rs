use std::fmt;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}
impl Square {
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
    use super::*;

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
}
