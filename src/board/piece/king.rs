use super::super::bitboard::{Bitboard, File};
use super::{Color, Piece, Ply, Precomputed, Square};
use crate::board::Board;
use crate::board::{CastlingKind, CastlingStatus};
use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug)]
pub struct King;

static ATTACKS: OnceLock<[Bitboard; 64]> = OnceLock::new();

impl Eq for King {}

impl Piece for King {
    const WHITE_SYMBOL: &'static str = "♚";
    const BLACK_SYMBOL: &'static str = "♔";

    fn get_moveset(square: Square, board: &Board, color: Color) -> Vec<Ply> {
        let same_pieces = match color {
            Color::White => board.bitboards.white_pieces,
            Color::Black => board.bitboards.black_pieces,
        };

        let move_mask = Self::get_attacks(square) & !same_pieces;
        let squares: Vec<Square> = move_mask.into();

        let mut moveset: Vec<Ply> = squares.into_iter().map(|s| Ply::new(square, s)).collect();
        if square == Square::from("e1") && color == Color::White {
            if board
                .castling_ability(CastlingKind::WhiteKingside)
                .expect("Tried to castle for the wrong side!")
                == CastlingStatus::Availiable
            {
                moveset.push(
                    Ply::builder(square, Square::from("g1"))
                        .castles(true)
                        .build(),
                );
            }
            if board
                .castling_ability(CastlingKind::WhiteQueenside)
                .expect("Tried to castle for the wrong side!")
                == CastlingStatus::Availiable
            {
                moveset.push(
                    Ply::builder(square, Square::from("c1"))
                        .castles(true)
                        .build(),
                );
            }
        }

        if square == Square::from("e8") && color == Color::Black {
            if board
                .castling_ability(CastlingKind::BlackKingside)
                .expect("Tried to castle for the wrong side!")
                == CastlingStatus::Availiable
            {
                moveset.push(
                    Ply::builder(square, Square::from("g8"))
                        .castles(true)
                        .build(),
                );
            }
            if board
                .castling_ability(CastlingKind::BlackQueenside)
                .expect("Tried to castle for the wrong side!")
                == CastlingStatus::Availiable
            {
                moveset.push(
                    Ply::builder(square, Square::from("c8"))
                        .castles(true)
                        .build(),
                );
            }
        }

        moveset
    }
}

impl Precomputed for King {
    fn init_attacks() -> [Bitboard; 64] {
        assert!(ATTACKS.get().is_none());
        let mut attacks = [Bitboard::new(0); 64];
        for (idx, attacks_at_square) in attacks.iter_mut().enumerate() {
            let origin = Bitboard::new(1 << idx);
            *attacks_at_square = ((origin << 7 | origin >> 1 | origin >> 9) & !(File::H as u64)) | // Left by 1 square
            ((origin << 9 | origin << 1 | origin >> 7) & !(File::A as u64)) | // Right by 1 square
            (origin << 8 | origin >> 8); // Up or down by 1
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
    use super::{Color, King, Piece, Ply, Square};
    use crate::board::boardbuilder::BoardBuilder;
    use crate::board::Kind;
    use crate::utils::tests::check_unique_equality;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_king_derived_traits() {
        let piece = King {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_king_display_white() {
        let output = King::WHITE_SYMBOL;
        let correct = "♚";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_display_black() {
        let output = King::BLACK_SYMBOL;
        let correct = "♔";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_get_piece_symbol_white() {
        let piece = Kind::King(Color::White);
        let correct = "♚";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_get_piece_symbol_black() {
        let piece = Kind::King(Color::Black);
        let correct = "♔";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_eq() {
        let left = Kind::King(Color::White);
        let right = Kind::King(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_king_neq() {
        let left = Kind::King(Color::White);
        let right = Kind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_neq_rev() {
        // Test if addition is commutative
        let right = Kind::King(Color::White);
        let left = Kind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_get_moveset_white_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("a1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_king_get_moveset_white_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("e5")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_white_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h7")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_black_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c2")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_black_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("e5")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_black_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h7")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_white_e1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("e2")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("f2")),
            Ply::builder(start_square, Square::from("g1"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .castles(true)
                .build(),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_black_e1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("e2")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("f2")),
            Ply::builder(start_square, Square::from("g1"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .castles(true)
                .build(),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_white_e8() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e8");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d8")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("e7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("f7")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_king_get_moveset_black_e8() {
        let mut board = BoardBuilder::construct_empty_board().build();
        board.switch_turn();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("e8");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d8")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("e7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("f7")),
            Ply::builder(start_square, Square::from("g8"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .castles(true)
                .build(),
        ];

        check_unique_equality(result, correct);
    }
}
