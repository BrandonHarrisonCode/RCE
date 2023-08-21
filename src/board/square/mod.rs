use derive_more::Constructor;
use std::fmt;

#[derive(Constructor, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
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
        let square = Square::new(3, 5);
        dbg!(&square);

        assert_eq!(square, square.clone());
    }

    #[test]
    fn test_display() {
        let square = Square::new(3, 5);

        let result = square.to_string();
        let correct = String::from("f4");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_rank() {
        let square = Square::new(9, 5);

        let result = square.to_string();
        let correct = String::from("Invalid range: 9, 5");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_file() {
        let square = Square::new(5, 9);

        let result = square.to_string();
        let correct = String::from("Invalid range: 5, 9");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_oob_both() {
        let square = Square::new(10, 19);

        let result = square.to_string();
        let correct = String::from("Invalid range: 10, 19");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_north() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::North.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_northeast() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::NorthEast.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_east() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::East.unit_square();

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_southeast() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::SouthEast.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file + 1, after.file);
    }

    #[test]
    fn test_south() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::South.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file, after.file);
    }

    #[test]
    fn test_southwest() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::SouthWest.unit_square();

        assert_eq!(before.rank - 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_west() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::West.unit_square();

        assert_eq!(before.rank, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_northwest() {
        let before = Square::new(4, 4);
        let after = before.clone() + Direction::NorthWest.unit_square();

        assert_eq!(before.rank + 1, after.rank);
        assert_eq!(before.file - 1, after.file);
    }

    #[test]
    fn test_direction_inverse() {
        let square = Square::new(4, 4);

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
}
