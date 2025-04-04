use super::super::bitboard::{Bitboard, File};
use super::{Color, Kind, Piece, Ply, Precomputed, Square};
use crate::board::Board;
use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug)]
pub struct Knight;

static ATTACKS: OnceLock<[Bitboard; 64]> = OnceLock::new();

impl Eq for Knight {}

impl Piece for Knight {
    const WHITE_SYMBOL: &'static str = "♞";
    const BLACK_SYMBOL: &'static str = "♘";

    fn get_moveset(square: Square, board: &Board, color: Color) -> Vec<Ply> {
        let same_pieces = match color {
            Color::White => board.bitboards.white_pieces,
            Color::Black => board.bitboards.black_pieces,
        };

        let move_mask = Self::get_attacks(square) & !same_pieces;
        let squares: Vec<Square> = move_mask.into();

        squares
            .into_iter()
            .map(|s| Ply::new(square, s, Kind::Knight(color)))
            .collect()
    }
}

impl Precomputed for Knight {
    fn init_attacks() -> [Bitboard; 64] {
        assert!(ATTACKS.get().is_none());
        let mut attacks = [Bitboard::new(0); 64];
        for (idx, attacks_at_square) in attacks.iter_mut().enumerate() {
            let origin = Bitboard::new(1 << idx);
            *attacks_at_square = (((origin << 15) | (origin >> 17)) & !(File::H as u64)) // Left by 1 square, up or down by 2
                | (((origin << 17) | (origin >> 15)) & !(File::A as u64)) // Right by 1 square, up or down by 2
                | (((origin << 10) | (origin >> 6)) & !(File::A as u64 | File::B as u64 )) // Right by 2 squares, up or down by 1
                | (((origin << 6) | (origin >> 10)) & !(File::G as u64 | File::H as u64));
            // Left by 2 squares, up or down by 1
        }

        attacks
    }

    fn get_attacks(square: Square) -> Bitboard {
        ATTACKS.get_or_init(Self::init_attacks)[square.u8() as usize]
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Knight, Piece, Ply, Square};
    use crate::board::BoardBuilder;
    use crate::board::Kind;
    use crate::testing_utils::tests::check_unique_equality;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashSet;

    #[test]
    fn test_knight_derived_traits() {
        let piece = Knight {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_knight_display_white() {
        let output = Knight::WHITE_SYMBOL;
        let correct = "♞";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_display_black() {
        let output = Knight::BLACK_SYMBOL;
        let correct = "♘";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_white() {
        let piece = Kind::Knight(Color::White);
        let correct = "♞";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_black() {
        let piece = Kind::Knight(Color::Black);
        let correct = "♘";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_eq() {
        let left = Kind::Knight(Color::White);
        let right = Kind::Knight(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_knight_neq() {
        let left = Kind::Knight(Color::White);
        let right = Kind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Knight(Color::White);
        let left = Kind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_get_moveset_white_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a3"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
        ];

        check_unique_equality(&result, &correct);
    }

    #[test]
    fn test_knight_get_moveset_white_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2"), piece), // Down 2, Left 1
            Ply::new(start_square, Square::from("e2"), piece), // Down 2, Right 1
            Ply::new(start_square, Square::from("c6"), piece), // Up 2, Left 1
            Ply::new(start_square, Square::from("e6"), piece), // Up 2, Right 1
            Ply::new(start_square, Square::from("b5"), piece), // Left 2, Up 1
            Ply::new(start_square, Square::from("b3"), piece), // Left 2, Down 1
            Ply::new(start_square, Square::from("f5"), piece), // Right 2, Up 1
            Ply::new(start_square, Square::from("f3"), piece), // Right 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_white_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g4"), piece), // Down 2, Left 1
            Ply::new(start_square, Square::from("g8"), piece), // Up 2, Left 1
            Ply::new(start_square, Square::from("f7"), piece), // Left 2, Up 1
            Ply::new(start_square, Square::from("f5"), piece), // Left 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a3"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
        ];

        check_unique_equality(&result, &correct);
    }

    #[test]
    fn test_knight_get_moveset_black_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2"), piece), // Down 2, Left 1
            Ply::new(start_square, Square::from("e2"), piece), // Down 2, Right 1
            Ply::new(start_square, Square::from("c6"), piece), // Up 2, Left 1
            Ply::new(start_square, Square::from("e6"), piece), // Up 2, Right 1
            Ply::new(start_square, Square::from("b5"), piece), // Left 2, Up 1
            Ply::new(start_square, Square::from("b3"), piece), // Left 2, Down 1
            Ply::new(start_square, Square::from("f5"), piece), // Right 2, Up 1
            Ply::new(start_square, Square::from("f3"), piece), // Right 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g4"), piece), // Down 2, Left 1
            Ply::new(start_square, Square::from("g8"), piece), // Up 2, Left 1
            Ply::new(start_square, Square::from("f7"), piece), // Left 2, Up 1
            Ply::new(start_square, Square::from("f5"), piece), // Left 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
