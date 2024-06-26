use super::super::bitboard::{Bitboard, File};
use super::{Color, Direction, Kind, Piece, Ply, PrecomputedColor, Square};
use crate::board::Board;
use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn;

static ATTACKS: OnceLock<[[Bitboard; 64]; 2]> = OnceLock::new();

impl Eq for Pawn {}

impl Pawn {
    fn explode_promotion(ply: Ply, color: Color, back_rank: u8) -> Vec<Ply> {
        if ply.dest.rank == back_rank {
            vec![
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Queen(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Rook(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Knight(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Bishop(color))
                    .build(),
            ]
        } else {
            vec![ply]
        }
    }
}

impl Piece for Pawn {
    const WHITE_SYMBOL: &'static str = "♟";
    const BLACK_SYMBOL: &'static str = "♙";

    fn get_moveset(square: Square, board: &Board, color: Color) -> Vec<Ply> {
        let (direction, starting_rank, en_passant_rank, back_rank) = match color {
            Color::White => (Direction::North, 1, 4, 7),
            Color::Black => (Direction::South, 6, 3, 0),
        };

        // Directional captures
        let move_mask = Self::get_attacks(square, color);
        let squares: Vec<Square> = move_mask.into();

        let mut moveset: Vec<Ply> = squares.into_iter().map(|s| Ply::new(square, s)).collect();

        // Single pawn push
        moveset.push(Ply::new(square, square + direction));
        // Double pawn push
        let offset: i32 = match color {
            Color::White => 8,
            Color::Black => -8,
        };
        if square.rank == starting_rank
            && (board.bitboards.all_pieces
                & (Bitboard::new(1) << (i32::from(u8::from(square)) + offset) as u32))
                .is_empty()
        {
            moveset.push(
                Ply::builder(square, square + direction + direction)
                    .double_pawn_push(true)
                    .build(),
            );
        }

        // En Passant
        if square.rank == en_passant_rank {
            moveset.push(
                Ply::builder(square, square + direction + Direction::East)
                    .en_passant(true)
                    .captured(Kind::Pawn(color.opposite()))
                    .build(),
            );
            moveset.push(
                Ply::builder(square, square + direction + Direction::West)
                    .en_passant(true)
                    .captured(Kind::Pawn(color.opposite()))
                    .build(),
            );
        }

        // Promotion
        moveset
            .iter()
            .flat_map(|ply| Self::explode_promotion(*ply, color, back_rank))
            .collect()
    }
}

impl PrecomputedColor for Pawn {
    fn init_attacks() -> [[Bitboard; 64]; 2] {
        assert!(ATTACKS.get().is_none());
        let mut attacks = [[Bitboard::new(0); 64]; 2];
        for idx in 0..64u8 {
            let origin = Bitboard::new(1 << idx);
            attacks[Color::White as usize][idx as usize] =
                ((origin << 9) & !(File::A as u64)) | ((origin << 7) & !(File::H as u64));
            attacks[Color::Black as usize][idx as usize] =
                ((origin >> 9) & !(File::H as u64)) | ((origin >> 7) & !(File::A as u64));
        }

        attacks
    }

    fn get_attacks(square: Square, color: Color) -> Bitboard {
        ATTACKS.get_or_init(Self::init_attacks)[color as usize][square.u8() as usize]
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Pawn, Piece, Ply, Square};
    use crate::board::boardbuilder::BoardBuilder;
    use crate::board::Kind;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashSet;

    #[test]
    fn test_pawn_derived_traits() {
        let piece = Pawn {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_pawn_display_white() {
        let output = Pawn::WHITE_SYMBOL;
        let correct = "♟";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_display_black() {
        let output = Pawn::BLACK_SYMBOL;
        let correct = "♙";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_white() {
        let piece = Kind::Pawn(Color::White);
        let correct = "♟";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_black() {
        let piece = Kind::Pawn(Color::Black);
        let correct = "♙";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_eq() {
        let left = Kind::Pawn(Color::White);
        let right = Kind::Pawn(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_pawn_neq() {
        let left = Kind::Pawn(Color::White);
        let right = Kind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Pawn(Color::White);
        let left = Kind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_get_moveset_white_a2() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("a2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a3")),
            Ply::new(start_square, Square::from("b3")),
            Ply::builder(start_square, Square::from("a4"))
                .double_pawn_push(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_d2() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("d2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::builder(start_square, Square::from("d4"))
                .double_pawn_push(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_h6() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("g7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_a3() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("a3");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("b2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_d5() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("d5");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_h7() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("h7");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h6")),
            Ply::builder(start_square, Square::from("h5"))
                .double_pawn_push(true)
                .build(),
            Ply::new(start_square, Square::from("g6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_h7() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("h7");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::builder(start_square, Square::from("h8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("h8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("h8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("h8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("g8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("g8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("g8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("g8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_h2() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("h2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::builder(start_square, Square::from("h1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("h1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("h1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("h1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("g1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("g1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("g1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("g1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_d7() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("d7");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::builder(start_square, Square::from("d8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("d8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("d8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("d8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("e8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("e8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("e8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("e8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_d2() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("d2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::builder(start_square, Square::from("d1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("d1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("d1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("d1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("e1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("e1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("e1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("e1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
