use std::fmt;

#[derive(Clone, PartialEq, Hash, Display, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Hash, Display, Debug)]
pub enum PieceKind {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }
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
}
