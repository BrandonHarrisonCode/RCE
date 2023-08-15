use std::fmt;

use super::ply::Ply;
use super::square::{Direction, Square};

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

#[derive(Clone, Copy, PartialEq, Hash, Display, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
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
    pub fn get_color(&self) -> Color {
        match self {
            PieceKind::Pawn(c) => *c,
            PieceKind::King(c) => *c,
            PieceKind::Queen(c) => *c,
            PieceKind::Rook(c) => *c,
            PieceKind::Bishop(c) => *c,
            PieceKind::Knight(c) => *c,
        }
    }

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

    fn get_moveset(&self, square: &Square) -> Vec<Ply> {
        match self {
            PieceKind::Pawn(_c) => Pawn::get_moveset(square),
            PieceKind::King(_c) => King::get_moveset(square),
            PieceKind::Queen(_c) => Queen::get_moveset(square),
            PieceKind::Rook(_c) => Rook::get_moveset(square),
            PieceKind::Bishop(_c) => Bishop::get_moveset(square),
            PieceKind::Knight(_c) => Knight::get_moveset(square),
        }
    }

    pub fn get_all_legal_moves(&self, square: &Square) -> Vec<Ply> {
        self.get_moveset(square)
            .into_iter()
            .filter(|mv| {
                mv.start.rank < 8
                    && mv.start.file < 8
                    && mv.dest.rank < 8
                    && mv.dest.file < 8
                    && mv.start != mv.dest
            })
            .collect()
    }
}

pub trait Piece: Clone + PartialEq + Eq {
    fn get_piece_symbol(color: &Color) -> &'static str;
    // Assumes always white?
    fn get_moveset(square: &Square) -> Vec<Ply>;
}

////////////////////////////////////////////////////////////////////////////////

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
