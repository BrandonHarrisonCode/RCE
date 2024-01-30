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

    fn get_moveset(square: &Square, _: &Color) -> Vec<Ply> {
        let move_mask = square.get_rank_mask() | square.get_file_mask();
        let squares = Square::get_squares_from_mask(move_mask);

        squares.into_iter().map(|s| Ply::new(*square, s)).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    fn test_rook_get_moveset_white_b1() {
        let piece = PieceKind::Rook(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("b3")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("b5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("b8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_white_d4() {
        let piece = PieceKind::Rook(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("d8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_white_h6() {
        let piece = PieceKind::Rook(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("h2")),
            Ply::new(start_square, Square::new("h3")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_b1() {
        let piece = PieceKind::Rook(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("b3")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("b5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("b8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_d4() {
        let piece = PieceKind::Rook(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("d8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_h6() {
        let piece = PieceKind::Rook(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("h2")),
            Ply::new(start_square, Square::new("h3")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
