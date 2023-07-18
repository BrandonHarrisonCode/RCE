use std::fmt;

#[derive(Clone, PartialEq, Hash, Display)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Hash, Display)]
pub enum PieceKind {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

pub fn build_piece(color: Color, kind: PieceKind) -> Piece {
    Piece { color, kind }
}

#[derive(Clone, PartialEq, Hash)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}
impl Eq for Piece {}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_symbol = match *self {
            Piece {
                color: Color::White,
                kind: PieceKind::Pawn,
            } => "♟",
            Piece {
                color: Color::White,
                kind: PieceKind::King,
            } => "♚",
            Piece {
                color: Color::White,
                kind: PieceKind::Queen,
            } => "♛",
            Piece {
                color: Color::White,
                kind: PieceKind::Rook,
            } => "♜",
            Piece {
                color: Color::White,
                kind: PieceKind::Bishop,
            } => "♝",
            Piece {
                color: Color::White,
                kind: PieceKind::Knight,
            } => "♞",
            Piece {
                color: Color::Black,
                kind: PieceKind::Pawn,
            } => "♙",
            Piece {
                color: Color::Black,
                kind: PieceKind::King,
            } => "♔",
            Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
            } => "♕",
            Piece {
                color: Color::Black,
                kind: PieceKind::Rook,
            } => "♖",
            Piece {
                color: Color::Black,
                kind: PieceKind::Bishop,
            } => "♗",
            Piece {
                color: Color::Black,
                kind: PieceKind::Knight,
            } => "♘",
        };
        write!(f, "{}", piece_symbol)
    }
}
