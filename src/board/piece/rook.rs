use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Rook;

const WHITE_SYMBOL: &str = "♜";
const BLACK_SYMBOL: &str = "♖";

impl Eq for Rook {}

impl Piece for Rook {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(_square: &Square) -> Vec<Move> {
        Vec::new()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_derived_traits() {
        let piece = Rook {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_rook_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♜";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_rook_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♖";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_rook_get_piece_symbol_white() {
        let piece = PieceKind::Rook(Color::White);
        let correct = "♜";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_rook_get_piece_symbol_black() {
        let piece = PieceKind::Rook(Color::Black);
        let correct = "♖";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_rook_eq() {
        let left = PieceKind::Rook(Color::White);
        let right = PieceKind::Rook(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_rook_neq() {
        let left = PieceKind::Rook(Color::White);
        let right = PieceKind::Rook(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_rook_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Rook(Color::White);
        let left = PieceKind::Rook(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_rook_get_moveset() {
        let piece = PieceKind::Rook(Color::White);
        let result = piece.get_moveset(&Square::new(0, 0));
        let correct = Vec::new();

        assert_eq!(result, correct);
    }
}
