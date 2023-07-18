use derive_more::Constructor;
use std::fmt;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

#[derive(Constructor, Clone, Debug)]
pub struct Square {
    rank: u8,
    file: u8,
}
impl std::ops::Add<SquareDelta> for Square {
    type Output = Square;

    fn add(self, other: SquareDelta) -> Square {
        Square {
            rank: self.rank.saturating_add_signed(other.rank_delta),
            file: self.file.saturating_add_signed(other.file_delta),
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let filechar: char = (65 + self.rank) as char;
        write!(f, "{}{}", filechar, self.rank)
    }
}

pub struct SquareDelta {
    rank_delta: i8,
    file_delta: i8,
}

#[derive(Constructor, Debug)]
pub struct Move {
    starting_square: Square,
    target_square: Square,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.starting_square, self.target_square)
    }
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
                rank_delta: -1,
                file_delta: 0,
            },
            Self::NorthWest => SquareDelta {
                rank_delta: 1,
                file_delta: -1,
            },
        }
    }
}

#[derive(Clone, PartialEq, Hash, Display, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub enum PieceKind {
    Pawn(Color),
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
}

impl Eq for PieceKind {}

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_piece_symbol())
    }
}

impl PieceKind {
    pub fn get_piece_symbol(&self) -> &'static str {
        match self {
            PieceKind::Pawn(c) => Pawn::get_piece_symbol(c),
            PieceKind::King(c) => King::get_piece_symbol(c),
            PieceKind::Queen(c) => Queen::get_piece_symbol(c),
            PieceKind::Rook(c) => Rook::get_piece_symbol(c),
            PieceKind::Bishop(c) => Bishop::get_piece_symbol(c),
            PieceKind::Knight(c) => Knight::get_piece_symbol(c),
        }
    }

    pub fn get_all_moves(&self, rank: u8, file: u8) -> Vec<Move> {
        match self {
            PieceKind::Pawn(_c) => Pawn::get_all_moves(rank, file),
            PieceKind::King(_c) => King::get_all_moves(rank, file),
            PieceKind::Queen(_c) => Queen::get_all_moves(rank, file),
            PieceKind::Rook(_c) => Rook::get_all_moves(rank, file),
            PieceKind::Bishop(_c) => Bishop::get_all_moves(rank, file),
            PieceKind::Knight(_c) => Knight::get_all_moves(rank, file),
        }
    }
}

pub trait Piece: Clone + PartialEq + Eq {
    fn get_piece_symbol(color: &Color) -> &'static str;
    // Assumes always white?
    fn get_all_moves(rank: u8, file: u8) -> Vec<Move>;
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        PieceKind::Pawn(Color::White).to_string();
        PieceKind::King(Color::White).to_string();
        PieceKind::Queen(Color::White).to_string();
        PieceKind::Rook(Color::White).to_string();
        PieceKind::Bishop(Color::White).to_string();
        PieceKind::Knight(Color::White).to_string();

        PieceKind::Pawn(Color::Black).to_string();
        PieceKind::King(Color::Black).to_string();
        PieceKind::Queen(Color::Black).to_string();
        PieceKind::Rook(Color::Black).to_string();
        PieceKind::Bishop(Color::Black).to_string();
        PieceKind::Knight(Color::Black).to_string();
    }

    #[test]
    fn check_eq() {
        let piece1 = PieceKind::Pawn(Color::White);
        let piece2 = PieceKind::Pawn(Color::White);
        assert_eq!(piece1, piece2);
    }

    #[test]
    fn check_ne_color() {
        let piece1 = PieceKind::Pawn(Color::White);
        let piece2 = PieceKind::Pawn(Color::Black);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_kind() {
        let piece1 = PieceKind::Pawn(Color::White);
        let piece2 = PieceKind::Queen(Color::White);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_both() {
        let piece1 = PieceKind::Pawn(Color::White);
        let piece2 = PieceKind::Queen(Color::Black);
        assert_ne!(piece1, piece2);
    }
}
