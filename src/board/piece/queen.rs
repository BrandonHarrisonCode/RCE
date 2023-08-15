use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Queen;

const WHITE_SYMBOL: &str = "♛";
const BLACK_SYMBOL: &str = "♕";

impl Eq for Queen {}

impl Piece for Queen {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(_square: &Square) -> Vec<Ply> {
        Vec::new()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queen_derived_traits() {
        let piece = Queen {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_queen_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♛";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_queen_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♕";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_queen_get_piece_symbol_white() {
        let piece = PieceKind::Queen(Color::White);
        let correct = "♛";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_queen_get_piece_symbol_black() {
        let piece = PieceKind::Queen(Color::Black);
        let correct = "♕";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_queen_eq() {
        let left = PieceKind::Queen(Color::White);
        let right = PieceKind::Queen(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_queen_neq() {
        let left = PieceKind::Queen(Color::White);
        let right = PieceKind::Queen(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_queen_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Queen(Color::White);
        let left = PieceKind::Queen(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_queen_get_moveset() {
        let piece = PieceKind::Queen(Color::White);
        let result = piece.get_moveset(&Square::new(0, 0));
        let correct = Vec::new();

        assert_eq!(result, correct);
    }
}
