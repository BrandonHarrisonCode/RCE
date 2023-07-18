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
    fn new_piece() {
        let result = Piece::new(Color::Black, PieceKind::Rook);
        assert_eq!(
            result,
            Piece {
                color: Color::Black,
                kind: PieceKind::Rook
            }
        );
    }

    #[test]
    fn check_eq() {
        let piece1 = Piece::new(Color::White, PieceKind::Queen);
        let piece2 = Piece::new(Color::White, PieceKind::Queen);
        assert_eq!(piece1, piece2);
    }

    #[test]
    fn check_ne_color() {
        let piece1 = Piece::new(Color::White, PieceKind::Queen);
        let piece2 = Piece::new(Color::Black, PieceKind::Queen);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_kind() {
        let piece1 = Piece::new(Color::White, PieceKind::Queen);
        let piece2 = Piece::new(Color::White, PieceKind::King);
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn check_ne_both() {
        let piece1 = Piece::new(Color::White, PieceKind::Queen);
        let piece2 = Piece::new(Color::Black, PieceKind::King);
        assert_ne!(piece1, piece2);
    }
}
