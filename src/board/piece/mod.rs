pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Clone, PartialEq, Hash, Display, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Hash, Display, Debug)]
pub enum PieceKind {
    Pawn(Color),
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
}
impl Eq for PieceKind {}

pub trait Piece: Clone + PartialEq + Eq {
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
