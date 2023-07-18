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
