use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn;

const WHITE_SYMBOL: &str = "♟";
const BLACK_SYMBOL: &str = "♙";

impl Eq for Pawn {}

impl Piece for Pawn {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    /// [X] Advances 1 square forward
    /// [ ] Advances 2 squares forward if on second rank
    /// [ ] Takes diagonally forward
    /// [ ] En passant
    /// [ ] Promotion
    fn get_moveset(square: &Square) -> Vec<Move> {
        let mut output: Vec<Move> = Vec::new();
        output.push(Move::new(
            square.clone(),
            square.clone() + Direction::North.unit_square(),
        ));
        output
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_derived_traits() {
        let piece = Pawn {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_pawn_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♟";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♙";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_white() {
        let piece = PieceKind::Pawn(Color::White);
        let correct = "♟";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_black() {
        let piece = PieceKind::Pawn(Color::Black);
        let correct = "♙";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_eq() {
        let left = PieceKind::Pawn(Color::White);
        let right = PieceKind::Pawn(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_pawn_neq() {
        let left = PieceKind::Pawn(Color::White);
        let right = PieceKind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Pawn(Color::White);
        let left = PieceKind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_get_moveset_white_a2() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new(1, 0);
        let dest_square = Square::new(2, 0);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Move::new(start_square, dest_square));

        assert_eq!(result, correct);
    }

    #[test]
    fn test_pawn_get_moveset_white_d2() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new(1, 3);
        let dest_square = Square::new(2, 3);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Move::new(start_square, dest_square));

        assert_eq!(result, correct);
    }

    #[test]
    fn test_pawn_get_moveset_white_h6() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new(5, 7);
        let dest_square = Square::new(6, 7);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Move::new(start_square, dest_square));

        assert_eq!(result, correct);
    }
}
