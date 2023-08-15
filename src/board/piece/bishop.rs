use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop;

const WHITE_SYMBOL: &str = "♝";
const BLACK_SYMBOL: &str = "♗";

impl Eq for Bishop {}

impl Piece for Bishop {
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
    fn test_bishop_derived_traits() {
        let piece = Bishop {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_bishop_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♝";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♗";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_white() {
        let piece = PieceKind::Bishop(Color::White);
        let correct = "♝";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_black() {
        let piece = PieceKind::Bishop(Color::Black);
        let correct = "♗";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_eq() {
        let left = PieceKind::Bishop(Color::White);
        let right = PieceKind::Bishop(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_bishop_neq() {
        let left = PieceKind::Bishop(Color::White);
        let right = PieceKind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Bishop(Color::White);
        let left = PieceKind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_get_moveset() {
        let piece = PieceKind::Bishop(Color::White);
        let result = piece.get_moveset(&Square::new(0, 0));
        let correct = Vec::new();

        assert_eq!(result, correct);
    }
}
