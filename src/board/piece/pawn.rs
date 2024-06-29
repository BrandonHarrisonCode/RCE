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
        const NEXT_SQUARE_OFFSET: usize = 8;
        const DOUBLE_NEXT_SQUARE_OFFSET: usize = 2 * NEXT_SQUARE_OFFSET;

        let (direction, starting_rank, en_passant_rank, back_rank) = match color {
            Color::White => (Direction::North, 1, 4, 7),
            Color::Black => (Direction::South, 6, 3, 0),
        };

        let enemy_pieces = match color {
            Color::White => board.bitboards.black_pieces,
            Color::Black => board.bitboards.white_pieces,
        };

        // Directional captures
        let move_mask = Self::get_attacks(square, color) & enemy_pieces;
        let squares: Vec<Square> = move_mask.into();

        let mut moveset: Vec<Ply> = squares.into_iter().map(|s| Ply::new(square, s)).collect();

        #[allow(clippy::cast_possible_truncation)]
        let next_square_mask = match color {
            Color::White => {
                Bitboard::new(1) << u32::from(u8::from(square)) << NEXT_SQUARE_OFFSET as u32
            }
            Color::Black => Bitboard::new(1) << u32::from(u8::from(square)) >> NEXT_SQUARE_OFFSET,
        } & board.bitboards.all_pieces;

        #[allow(clippy::cast_possible_truncation)]
        let double_next_square_mask = match color {
            Color::White => {
                Bitboard::new(1) << u32::from(u8::from(square)) << DOUBLE_NEXT_SQUARE_OFFSET as u32
            }
            Color::Black => {
                Bitboard::new(1) << u32::from(u8::from(square)) >> DOUBLE_NEXT_SQUARE_OFFSET
            }
        } & board.bitboards.all_pieces;

        // Single pawn push
        if next_square_mask.is_empty() {
            moveset.push(Ply::new(square, square + direction));
        }

        // Double pawn push
        if square.rank == starting_rank
            && next_square_mask.is_empty()
            && double_next_square_mask.is_empty()
        {
            moveset.push(
                Ply::builder(square, square + direction + direction)
                    .double_pawn_push(true)
                    .build(),
            );
        }

        // En Passant
        if square.rank == en_passant_rank {
            let dest_east = square + direction + Direction::East;
            if board
                .en_passant_file
                .is_some_and(|file| file == dest_east.file)
            {
                moveset.push(
                    Ply::builder(square, dest_east)
                        .en_passant(true)
                        .captured(Kind::Pawn(color.opposite()))
                        .build(),
                );
            }

            let dest_west = square + direction + Direction::West;
            if board
                .en_passant_file
                .is_some_and(|file| file == dest_west.file)
            {
                moveset.push(
                    Ply::builder(square, dest_west)
                        .en_passant(true)
                        .captured(Kind::Pawn(color.opposite()))
                        .build(),
                );
            }
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
    use crate::board::Kind;
    use crate::board::{boardbuilder::BoardBuilder, Board};
    use crate::utils::tests::check_unique_equality;
    use pretty_assertions::{assert_eq, assert_ne};

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
        let board = BoardBuilder::construct_starting_board().build();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("a2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a3")),
            Ply::builder(start_square, Square::from("a4"))
                .double_pawn_push(true)
                .build(),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_white_d2() {
        let board = BoardBuilder::construct_starting_board().build();
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("d2");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d3")),
            Ply::builder(start_square, Square::from("d4"))
                .double_pawn_push(true)
                .build(),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_white_h6() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/7P/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![Ply::new(start_square, Square::from("g7"))];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_a3() {
        let board = Board::from_fen("rnbqkbnr/1ppppppp/8/8/8/p7/1PPPPPPP/RNBQKBNR w KQkq - 0 1");
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("a3");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("b2")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_d5() {
        let board = Board::from_fen("rnbqkbnr/1ppppppp/8/3p4/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 1");
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("d5");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("c4")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_h7() {
        let board = BoardBuilder::construct_starting_board().build();
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("h7");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h6")),
            Ply::builder(start_square, Square::from("h5"))
                .double_pawn_push(true)
                .build(),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_white_h7() {
        let board = Board::from_fen("rnbqkbn1/pppppppP/8/8/8/8/PPPPPPP1/RNBQKBNR w KQq - 0 1");
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

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_white_h7_2() {
        let board = Board::from_fen("rnbqkb2/pppppppP/8/8/8/8/PPPPPPP1/RNBQKBNR w KQq - 0 1");
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
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_h2() {
        let board = Board::from_fen("rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPPp/RNBQKBN1 w Qkq - 0 1");
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

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_h2_2() {
        let board = Board::from_fen("rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPPp/RNBQKB2 w Qkq - 0 1");
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
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_white_d7() {
        let board = Board::from_fen("rnb1r1k1/pppPpppp/8/8/8/8/PPP1PPPP/RNBQKBNR w KQq - 0 1");
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

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_black_d2() {
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/8/8/8/PPPpPPPP/RNB1R1K1 b Qkq - 0 1");
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

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_pawn_get_moveset_from_position_1() {
        let board = Board::from_fen("4r2k/4qpRb/2p1p2Q/1p3r1P/p2P4/P4P2/1PP1N3/1K4R1 b - - 2 32");
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::from("a4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![];

        check_unique_equality(result, correct)
    }
}
