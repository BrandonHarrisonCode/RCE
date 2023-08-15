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
