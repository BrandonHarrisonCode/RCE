use super::super::bitboard::Bitboard;
use super::{Color, Magic, Piece, Ply, Square};
use crate::board::square::rays::RAYS;
use crate::board::square::Direction;
use crate::board::Board;
use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop;

static MASKS: OnceLock<[Bitboard; 64]> = OnceLock::new();
static ATTACKS: OnceLock<[Vec<Bitboard>; 64]> = OnceLock::new();
const ATTACKS_TABLE_SIZE: usize = 1024; // TODO change to 512 when tests pass

impl Eq for Bishop {}

impl Piece for Bishop {
    const WHITE_SYMBOL: &'static str = "♝";
    const BLACK_SYMBOL: &'static str = "♗";

    fn get_moveset(square: Square, board: &Board, _: Color) -> Vec<Ply> {
        let move_mask = Self::get_attacks(square, board.bitboards.all_pieces);
        let squares: Vec<Square> = move_mask.into();

        squares.into_iter().map(|s| Ply::new(square, s)).collect()
    }
}

impl Magic for Bishop {
    fn init_masks() -> [Bitboard; 64] {
        assert!(MASKS.get().is_none());
        let mut masks: [Bitboard; 64] = [Bitboard::new(0); 64];
        let rays = RAYS.get_or_init(crate::board::square::rays::Rays::new).rays;

        for i in 0..64u8 {
            let mask: Bitboard = (rays[i as usize][Direction::NorthEast as usize]
                | rays[i as usize][Direction::SouthEast as usize]
                | rays[i as usize][Direction::SouthWest as usize]
                | rays[i as usize][Direction::NorthWest as usize])
                .trim_edges();

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

        let northwest_ray = rays[square.u8() as usize][Direction::NorthWest as usize];
        let northeast_ray = rays[square.u8() as usize][Direction::NorthEast as usize];
        let southwest_ray = rays[square.u8() as usize][Direction::SouthWest as usize];
        let southeast_ray = rays[square.u8() as usize][Direction::SouthEast as usize];

        let mut attacks = northwest_ray | northeast_ray | southwest_ray | southeast_ray;

        if !(northwest_ray & blockers).is_empty() {
            let blocked_idx = (northwest_ray & blockers).bitscan_forward();
            attacks &= !(rays[blocked_idx as usize][Direction::NorthWest as usize]);
        }

        if !(northeast_ray & blockers).is_empty() {
            let blocked_idx = (northeast_ray & blockers).bitscan_forward();
            attacks &= !(rays[blocked_idx as usize][Direction::NorthEast as usize]);
        }

        if !(southwest_ray & blockers).is_empty() {
            let blocked_idx = (southwest_ray & blockers).bitscan_reverse();
            attacks &= !(rays[blocked_idx as usize][Direction::SouthWest as usize]);
        }

        if !(southeast_ray & blockers).is_empty() {
            let blocked_idx = (southeast_ray & blockers).bitscan_reverse();
            attacks &= !(rays[blocked_idx as usize][Direction::SouthEast as usize]);
        }

        attacks
    }
}

impl Bishop {
    #[allow(clippy::unreadable_literal)]
    const MAGICS: [u64; 64] = [
        0x89a1121896040240,
        0x2004844802002010,
        0x2068080051921000,
        0x62880a0220200808,
        0x4042004000000,
        0x100822020200011,
        0xc00444222012000a,
        0x28808801216001,
        0x400492088408100,
        0x201c401040c0084,
        0x840800910a0010,
        0x82080240060,
        0x2000840504006000,
        0x30010c4108405004,
        0x1008005410080802,
        0x8144042209100900,
        0x208081020014400,
        0x4800201208ca00,
        0xf18140408012008,
        0x1004002802102001,
        0x841000820080811,
        0x40200200a42008,
        0x800054042000,
        0x88010400410c9000,
        0x520040470104290,
        0x1004040051500081,
        0x2002081833080021,
        0x400c00c010142,
        0x941408200c002000,
        0x658810000806011,
        0x188071040440a00,
        0x4800404002011c00,
        0x104442040404200,
        0x511080202091021,
        0x4022401120400,
        0x80c0040400080120,
        0x8040010040820802,
        0x480810700020090,
        0x102008e00040242,
        0x809005202050100,
        0x8002024220104080,
        0x431008804142000,
        0x19001802081400,
        0x200014208040080,
        0x3308082008200100,
        0x41010500040c020,
        0x4012020c04210308,
        0x208220a202004080,
        0x111040120082000,
        0x6803040141280a00,
        0x2101004202410000,
        0x8200000041108022,
        0x21082088000,
        0x2410204010040,
        0x40100400809000,
        0x822088220820214,
        0x40808090012004,
        0x910224040218c9,
        0x402814422015008,
        0x90014004842410,
        0x1000042304105,
        0x10008830412a00,
        0x2520081090008908,
        0x40102000a0a60140,
    ];

    const INDEX_BITS: [u8; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7,
        5, 5, 5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5,
        5, 5, 5, 6,
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
    use super::{Bishop, Color, Piece, Ply, Square};
    use crate::board::Kind;
    use crate::{board::boardbuilder::BoardBuilder, utils::tests::check_unique_equality};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_bishop_derived_traits() {
        let piece = Bishop {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_bishop_display_white() {
        let output = Bishop::WHITE_SYMBOL;
        let correct = "♝";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_display_black() {
        let output = Bishop::BLACK_SYMBOL;
        let correct = "♗";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_white() {
        let piece = Kind::Bishop(Color::White);
        let correct = "♝";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_black() {
        let piece = Kind::Bishop(Color::Black);
        let correct = "♗";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_eq() {
        let left = Kind::Bishop(Color::White);
        let right = Kind::Bishop(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_bishop_neq() {
        let left = Kind::Bishop(Color::White);
        let right = Kind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Bishop(Color::White);
        let left = Kind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_get_moveset_white_a1() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_white_b1() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("a2")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_white_e4() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("a8")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("f3")),
            Ply::new(start_square, Square::from("g2")),
            Ply::new(start_square, Square::from("h1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_white_d4() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("a7")),
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("f2")),
            Ply::new(start_square, Square::from("g1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_white_g6() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("f7")),
            Ply::new(start_square, Square::from("e8")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("b1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_white_h6() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("c1")),
        ];
        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_a1() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_b1() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("a2")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_e4() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("a8")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("f3")),
            Ply::new(start_square, Square::from("g2")),
            Ply::new(start_square, Square::from("h1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_d4() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("a7")),
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("f2")),
            Ply::new(start_square, Square::from("g1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_g6() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("f7")),
            Ply::new(start_square, Square::from("e8")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("b1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_h6() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("c1")),
        ];

        check_unique_equality(result, correct)
    }

    #[test]
    fn test_bishop_get_moveset_black_e8() {
        let board = BoardBuilder::construct_empty_board();
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("e8");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a4")),
            Ply::new(start_square, Square::from("b5")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("f7")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h5")),
        ];

        check_unique_equality(result, correct)
    }
}
