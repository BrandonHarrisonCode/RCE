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

    fn get_moveset(square: &Square, _: &Color) -> Vec<Ply> {
        let move_mask =
            square.get_rank_mask() | square.get_file_mask() | square.get_diagonals_mask();
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
    fn test_queen_get_moveset_white_a1() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("a1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d4")),
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("a5")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("a7")),
            Ply::new(start_square, Square::new("a8")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_b1() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("b3")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("b5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("b8")),
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_e4() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("e4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("a8")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("f3")),
            Ply::new(start_square, Square::new("g2")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("e2")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("e7")),
            Ply::new(start_square, Square::new("e8")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("d4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_d4() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("c5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("a7")),
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("f2")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("d8")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_g6() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("g6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("f7")),
            Ply::new(start_square, Square::new("e8")),
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("g2")),
            Ply::new(start_square, Square::new("g3")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("g8")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("h6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_h6() {
        let piece = PieceKind::Queen(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("f8")),
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("h2")),
            Ply::new(start_square, Square::new("h3")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_a1() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("a1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d4")),
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("a5")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("a7")),
            Ply::new(start_square, Square::new("a8")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_b1() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("b3")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("b5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("b8")),
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_e4() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("e4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("b7")),
            Ply::new(start_square, Square::new("a8")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("f3")),
            Ply::new(start_square, Square::new("g2")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("e1")),
            Ply::new(start_square, Square::new("e2")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("e7")),
            Ply::new(start_square, Square::new("e8")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("d4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_d4() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("e5")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("c5")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("a7")),
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("f2")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("d8")),
            Ply::new(start_square, Square::new("a4")),
            Ply::new(start_square, Square::new("b4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("h4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_g6() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("g6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("f7")),
            Ply::new(start_square, Square::new("e8")),
            Ply::new(start_square, Square::new("f5")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("b1")),
            Ply::new(start_square, Square::new("g1")),
            Ply::new(start_square, Square::new("g2")),
            Ply::new(start_square, Square::new("g3")),
            Ply::new(start_square, Square::new("g4")),
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("g8")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("h6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_h6() {
        let piece = PieceKind::Queen(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("f8")),
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("f4")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("h1")),
            Ply::new(start_square, Square::new("h2")),
            Ply::new(start_square, Square::new("h3")),
            Ply::new(start_square, Square::new("h4")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("h8")),
            Ply::new(start_square, Square::new("a6")),
            Ply::new(start_square, Square::new("b6")),
            Ply::new(start_square, Square::new("c6")),
            Ply::new(start_square, Square::new("d6")),
            Ply::new(start_square, Square::new("e6")),
            Ply::new(start_square, Square::new("f6")),
            Ply::new(start_square, Square::new("g6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
