use std::fmt;

use super::bitboard::Bitboard;
use super::ply::Ply;
use super::square::{Direction, Square};

use crate::board::Board;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub const fn default() -> Self {
        Self::White
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub enum Kind {
    Pawn(Color),
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
}

impl Eq for Kind {}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_piece_symbol())
    }
}

impl Kind {
    pub const fn get_color(self) -> Color {
        match self {
            Self::Pawn(c)
            | Self::King(c)
            | Self::Queen(c)
            | Self::Rook(c)
            | Self::Bishop(c)
            | Self::Knight(c) => c,
        }
    }

    pub fn get_piece_symbol(self) -> &'static str {
        match self {
            Self::Pawn(c) => Pawn::get_piece_symbol(c),
            Self::King(c) => King::get_piece_symbol(c),
            Self::Queen(c) => Queen::get_piece_symbol(c),
            Self::Rook(c) => Rook::get_piece_symbol(c),
            Self::Bishop(c) => Bishop::get_piece_symbol(c),
            Self::Knight(c) => Knight::get_piece_symbol(c),
        }
    }

    pub fn get_moveset(self, square: Square, board: &Board) -> Vec<Ply> {
        let moveset = match self {
            Self::Pawn(color) => Pawn::get_moveset(square, board, color),
            Self::King(color) => King::get_moveset(square, board, color),
            Self::Queen(color) => Queen::get_moveset(square, board, color),
            Self::Rook(color) => Rook::get_moveset(square, board, color),
            Self::Bishop(color) => Bishop::get_moveset(square, board, color),
            Self::Knight(color) => Knight::get_moveset(square, board, color),
        };

        moveset
            .into_iter()
            .filter(|mv| {
                mv.start.rank < 8
                    && mv.start.file < 8
                    && mv.dest.rank < 8
                    && mv.dest.file < 8
                    && mv.start != mv.dest
            })
            .collect::<Vec<Ply>>()
    }
}

pub trait Piece: Clone + PartialEq + Eq {
    const WHITE_SYMBOL: &'static str;
    const BLACK_SYMBOL: &'static str;

    fn get_piece_symbol(color: Color) -> &'static str {
        match color {
            Color::White => Self::WHITE_SYMBOL,
            Color::Black => Self::BLACK_SYMBOL,
        }
    }

    fn get_moveset(square: Square, board: &Board, color: Color) -> Vec<Ply>;
}

trait Magic {
    fn init_masks() -> [Bitboard; 64];
    fn get_attacks(square: Square, blockers: Bitboard) -> Bitboard;
    fn get_attacks_slow(square: Square, blockers: Bitboard) -> Bitboard;

    fn get_blockers_from_index(idx: u16, mut mask: Bitboard) -> Bitboard {
        let mut blockers = Bitboard::new(0);
        let bits = mask.count_ones();
        for i in 0..bits {
            let bitidx = mask.drop_forward();
            if idx & (1 << i) != 0 {
                blockers |= 1 << bitidx;
            }
        }

        blockers
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_derived_traits_piece() {
        let piece = Kind::Pawn(Color::White);
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn display() {
        Kind::Pawn(Color::White).to_string();
        Kind::King(Color::White).to_string();
        Kind::Queen(Color::White).to_string();
        Kind::Rook(Color::White).to_string();
        Kind::Bishop(Color::White).to_string();
        Kind::Knight(Color::White).to_string();

        Kind::Pawn(Color::Black).to_string();
        Kind::King(Color::Black).to_string();
        Kind::Queen(Color::Black).to_string();
        Kind::Rook(Color::Black).to_string();
        Kind::Bishop(Color::Black).to_string();
        Kind::Knight(Color::Black).to_string();
    }

    #[test]
    fn test_get_color() {
        assert_eq!(Kind::Pawn(Color::White).get_color(), Color::White);
        assert_eq!(Kind::King(Color::White).get_color(), Color::White);
        assert_eq!(Kind::Queen(Color::White).get_color(), Color::White);
        assert_eq!(Kind::Rook(Color::White).get_color(), Color::White);
        assert_eq!(Kind::Bishop(Color::White).get_color(), Color::White);
        assert_eq!(Kind::Knight(Color::White).get_color(), Color::White);

        assert_eq!(Kind::Pawn(Color::Black).get_color(), Color::Black);
        assert_eq!(Kind::King(Color::Black).get_color(), Color::Black);
        assert_eq!(Kind::Queen(Color::Black).get_color(), Color::Black);
        assert_eq!(Kind::Rook(Color::Black).get_color(), Color::Black);
        assert_eq!(Kind::Bishop(Color::Black).get_color(), Color::Black);
        assert_eq!(Kind::Knight(Color::Black).get_color(), Color::Black);
    }

    #[test]
    fn check_eq() {
        let piece1 = Kind::Pawn(Color::White);
        let piece2 = Kind::Pawn(Color::White);
        assert_eq!(piece1, piece2);
    }

    #[test]
    fn check_ne_color() {
        let piece1 = Kind::Pawn(Color::White);
        let piece2 = Kind::Pawn(Color::Black);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_kind() {
        let piece1 = Kind::Pawn(Color::White);
        let piece2 = Kind::Queen(Color::White);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_both() {
        let piece1 = Kind::Pawn(Color::White);
        let piece2 = Kind::Queen(Color::Black);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn test_derived_traits_color() {
        let color = Color::White;
        dbg!(&color);
        println!("{color}");

        assert_eq!(color, color.clone());
    }
}
