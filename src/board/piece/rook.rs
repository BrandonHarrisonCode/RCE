use super::{Color, Piece, Ply, Square};

#[derive(Clone, PartialEq, Debug)]
pub struct Rook {
    rook_mask: [u64; 64],
}

impl Eq for Rook {}

impl Piece for Rook {
    const WHITE_SYMBOL: &'static str = "♜";
    const BLACK_SYMBOL: &'static str = "♖";

    fn get_moveset(square: Square, _: Color) -> Vec<Ply> {
        let move_mask = square.get_rank_mask() | square.get_file_mask();
        let squares = Square::get_squares_from_mask(move_mask);

        squares.into_iter().map(|s| Ply::new(square, s)).collect()
    }
}

impl Rook {
    pub fn new() -> Self {
        Self {
            rook_mask: Self::init_rook_masks(),
        }
    }

    fn init_rook_masks() -> [u64; 64] {
        let mut masks = [0; 64];
        for i in 0..64 {
            let mut mask = 0;
            let square = Square::from(i);

            let rank_mask = square.get_rank_mask();
            let file_mask = square.get_file_mask();

            mask |= rank_mask | file_mask;
        }

        masks
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Piece, Ply, Rook, Square};
    use crate::board::Kind;
    use std::collections::HashSet;

    #[test]
    fn test_rook_derived_traits() {
        let piece = Rook::new();
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_rook_display_white() {
        let output = Rook::WHITE_SYMBOL;
        let correct = "♜";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_rook_display_black() {
        let output = Rook::BLACK_SYMBOL;
        let correct = "♖";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_rook_get_piece_symbol_white() {
        let piece = Kind::Rook(Color::White);
        let correct = "♜";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_rook_get_piece_symbol_black() {
        let piece = Kind::Rook(Color::Black);
        let correct = "♖";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_rook_eq() {
        let left = Kind::Rook(Color::White);
        let right = Kind::Rook(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_rook_neq() {
        let left = Kind::Rook(Color::White);
        let right = Kind::Rook(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_rook_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Rook(Color::White);
        let left = Kind::Rook(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_rook_get_moveset_white_b1() {
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("e1")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("g1")),
            Ply::new(start_square, Square::from("h1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("b3")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("b5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("b8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_white_d4() {
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a4")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("g4")),
            Ply::new(start_square, Square::from("h4")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("d8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_white_h6() {
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a6")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("e6")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h1")),
            Ply::new(start_square, Square::from("h2")),
            Ply::new(start_square, Square::from("h3")),
            Ply::new(start_square, Square::from("h4")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_b1() {
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("e1")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("g1")),
            Ply::new(start_square, Square::from("h1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("b3")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("b5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("b8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_d4() {
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a4")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("g4")),
            Ply::new(start_square, Square::from("h4")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("d8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_rook_get_moveset_black_h6() {
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("a6")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("e6")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h1")),
            Ply::new(start_square, Square::from("h2")),
            Ply::new(start_square, Square::from("h3")),
            Ply::new(start_square, Square::from("h4")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
