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
        let symbol: &str = match self {
            PieceKind::Pawn(c) => Pawn::get_piece_symbol(c),
            PieceKind::King(c) => King::get_piece_symbol(c),
            PieceKind::Queen(c) => Queen::get_piece_symbol(c),
            PieceKind::Rook(c) => Rook::get_piece_symbol(c),
            PieceKind::Bishop(c) => Bishop::get_piece_symbol(c),
            PieceKind::Knight(c) => Knight::get_piece_symbol(c),
        };
        write!(f, "{}", symbol)
    }
}

pub trait Piece: Clone + PartialEq + Eq {
    fn get_piece_symbol(color: &Color) -> &'static str;
    // Assumes always white?
    fn get_all_moves<'a>(&self, rank: u8, file: u8) -> &'a Vec<u64>;
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
