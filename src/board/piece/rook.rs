use super::super::bitboard::{Bitboard, File, Rank};
use super::{Color, Magic, Piece, Ply, Square};
use crate::board::square::rays::RAYS;
use crate::board::square::Direction;
use crate::board::Board;
use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug)]
pub struct Rook;

static MASKS: OnceLock<[Bitboard; 64]> = OnceLock::new();
static ATTACKS: OnceLock<[Vec<Bitboard>; 64]> = OnceLock::new();
const ATTACKS_TABLE_SIZE: usize = 4096;

impl Eq for Rook {}

impl Piece for Rook {
    const WHITE_SYMBOL: &'static str = "♜";
    const BLACK_SYMBOL: &'static str = "♖";

    fn get_moveset(square: Square, board: &Board, _: Color) -> Vec<Ply> {
        let move_mask = Self::get_attacks(square, board.bitboards.all_pieces);
        let squares: Vec<Square> = move_mask.into();

        squares.into_iter().map(|s| Ply::new(square, s)).collect()
    }
}

impl Magic for Rook {
    fn init_masks() -> [Bitboard; 64] {
        assert!(MASKS.get().is_none());
        let mut masks: [Bitboard; 64] = [Bitboard::new(0); 64];
        let rays = RAYS.get_or_init(crate::board::square::rays::Rays::new).rays;

        for i in 0..64u8 {
            let mask: Bitboard = rays[i as usize][Direction::North as usize]
                & !(Rank::Eighth as u64)
                | rays[i as usize][Direction::East as usize] & !(File::H as u64)
                | rays[i as usize][Direction::South as usize] & !(Rank::First as u64)
                | rays[i as usize][Direction::West as usize] & !(File::A as u64);

            masks[i as usize] = mask;
        }

        masks
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_attacks(square: Square, blockers: Bitboard) -> Bitboard {
        let masked_blockers = blockers & MASKS.get_or_init(Self::init_masks)[square.u8() as usize];
        let key: u64 = ((masked_blockers * Self::MAGICS[square.u8() as usize])
            >> (64 - Self::INDEX_BITS[square.u8() as usize]).into())
        .into();

        ATTACKS.get_or_init(Self::init_attacks)[square.u8() as usize][key as usize]
    }

    fn get_attacks_slow(square: Square, blockers: Bitboard) -> Bitboard {
        let rays = RAYS.get_or_init(crate::board::square::rays::Rays::new).rays;

        let north_ray = rays[square.u8() as usize][Direction::North as usize];
        let east_ray = rays[square.u8() as usize][Direction::East as usize];
        let south_ray = rays[square.u8() as usize][Direction::South as usize];
        let west_ray = rays[square.u8() as usize][Direction::West as usize];

        let mut attacks = north_ray | east_ray | south_ray | west_ray;

        if !(north_ray & blockers).is_empty() {
            let blocked_idx = (north_ray & blockers).bitscan_forward();
            attacks &= !(rays[blocked_idx as usize][Direction::North as usize]);
        }

        if !(east_ray & blockers).is_empty() {
            let blocked_idx = (east_ray & blockers).bitscan_forward();
            attacks &= !(rays[blocked_idx as usize][Direction::East as usize]);
        }

        if !(south_ray & blockers).is_empty() {
            let blocked_idx = (south_ray & blockers).bitscan_reverse();
            attacks &= !(rays[blocked_idx as usize][Direction::South as usize]);
        }

        if !(west_ray & blockers).is_empty() {
            let blocked_idx = (west_ray & blockers).bitscan_reverse();
            attacks &= !(rays[blocked_idx as usize][Direction::West as usize]);
        }

        attacks
    }
}

impl Rook {
    #[allow(clippy::unreadable_literal)]
    const MAGICS: [u64; 64] = [
        0xa8002c000108020,
        0x6c00049b0002001,
        0x100200010090040,
        0x2480041000800801,
        0x280028004000800,
        0x900410008040022,
        0x280020001001080,
        0x2880002041000080,
        0xa000800080400034,
        0x4808020004000,
        0x2290802004801000,
        0x411000d00100020,
        0x402800800040080,
        0xb000401004208,
        0x2409000100040200,
        0x1002100004082,
        0x22878001e24000,
        0x1090810021004010,
        0x801030040200012,
        0x500808008001000,
        0xa08018014000880,
        0x8000808004000200,
        0x201008080010200,
        0x801020000441091,
        0x800080204005,
        0x1040200040100048,
        0x120200402082,
        0xd14880480100080,
        0x12040280080080,
        0x100040080020080,
        0x9020010080800200,
        0x813241200148449,
        0x491604001800080,
        0x100401000402001,
        0x4820010021001040,
        0x400402202000812,
        0x209009005000802,
        0x810800601800400,
        0x4301083214000150,
        0x204026458e001401,
        0x40204000808000,
        0x8001008040010020,
        0x8410820820420010,
        0x1003001000090020,
        0x804040008008080,
        0x12000810020004,
        0x1000100200040208,
        0x430000a044020001,
        0x280009023410300,
        0xe0100040002240,
        0x200100401700,
        0x2244100408008080,
        0x8000400801980,
        0x2000810040200,
        0x8010100228810400,
        0x2000009044210200,
        0x4080008040102101,
        0x40002080411d01,
        0x2005524060000901,
        0x502001008400422,
        0x489a000810200402,
        0x1004400080a13,
        0x4000011008020084,
        0x26002114058042,
    ];

    const INDEX_BITS: [u8; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10,
        11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10,
        10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
    ];

    #[allow(clippy::cast_possible_truncation)]
    fn init_attacks() -> [Vec<Bitboard>; 64] {
        assert!(ATTACKS.get().is_none());
        let mut attacks: [Vec<Bitboard>; 64] =
            core::array::from_fn(|_| Vec::<Bitboard>::with_capacity(ATTACKS_TABLE_SIZE));
        for square in 0..64u8 {
            let mut vector = vec![Bitboard::new(0); ATTACKS_TABLE_SIZE];
            for idx in 0u16..(1 << Self::INDEX_BITS[square as usize]) {
                let blockers: Bitboard = Self::get_blockers_from_index(
                    idx,
                    MASKS.get_or_init(Self::init_masks)[square as usize],
                );
                let second_index = (blockers.wrapping_mul(Self::MAGICS[square as usize]))
                    >> (64 - Self::INDEX_BITS[square as usize]);
                let value = Self::get_attacks_slow(Square::from(square), blockers);
                vector[second_index as usize] = value;
            }

            attacks[square as usize] = vector;
        }

        attacks
    }

    pub fn get_attacks_wrapper(square: Square, blockers: Bitboard) -> Bitboard {
        Self::get_attacks(square, blockers)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Piece, Ply, Rook, Square};
    use crate::board::{Board, Kind};
    use crate::utils::tests::check_unique_equality;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashSet;

    #[test]
    fn test_rook_derived_traits() {
        let piece = Rook {};
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
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
        let board = Board::construct_empty_board();
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
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
    fn test_rook_get_moveset_white_a1_blockers_1() {
        let mut board = Board::construct_empty_board();
        board.add_piece(Square::from("a3"), Kind::Pawn(Color::White));
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("a3")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("e1")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("g1")),
            Ply::new(start_square, Square::from("h1")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_rook_get_moveset_black_a1_blockers_1() {
        let mut board = Board::construct_empty_board();
        board.add_piece(Square::from("a3"), Kind::Pawn(Color::Black));
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("a3")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("e1")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("g1")),
            Ply::new(start_square, Square::from("h1")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_rook_get_moveset_white_d4_blockers_1() {
        let mut board = Board::construct_empty_board();
        board.add_piece(Square::from("a4"), Kind::Pawn(Color::White));
        board.add_piece(Square::from("d6"), Kind::Queen(Color::White));
        board.add_piece(Square::from("d1"), Kind::Rook(Color::White));
        board.add_piece(Square::from("a2"), Kind::Pawn(Color::White));
        let piece = Kind::Rook(Color::White);
        let start_square = Square::from("d4");
        dbg!(start_square.u8());

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("a4")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("g4")),
            Ply::new(start_square, Square::from("h4")),
        ];

        check_unique_equality(result, correct);
    }

    #[test]
    fn test_rook_get_moveset_black_d4_blockers_1() {
        let mut board = Board::construct_empty_board();
        board.add_piece(Square::from("a4"), Kind::Pawn(Color::Black));
        board.add_piece(Square::from("d6"), Kind::Queen(Color::Black));
        board.add_piece(Square::from("d1"), Kind::Rook(Color::Black));
        board.add_piece(Square::from("a2"), Kind::Pawn(Color::Black));
        let piece = Kind::Rook(Color::Black);
        let start_square = Square::from("d4");
        dbg!(start_square.u8());

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("d6")),
            Ply::new(start_square, Square::from("a4")),
            Ply::new(start_square, Square::from("b4")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("g4")),
            Ply::new(start_square, Square::from("h4")),
        ];

        check_unique_equality(result, correct);
    }
}
