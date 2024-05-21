use super::Direction;
use super::Square;
use std::sync::OnceLock;

use crate::board::bitboard::{Bitboard, File, Rank};

pub struct Rays {
    pub rays: [[Bitboard; 64]; 8],
}

impl Rays {
    pub fn new() -> Self {
        Self { rays: init_rays() }
    }
}

pub static RAYS: OnceLock<Rays> = OnceLock::new();

fn init_rays() -> [[Bitboard; 64]; 8] {
    let mut rays = [[Bitboard::new(0); 64]; 8];

    for idx in 0..64 {
        rays[Direction::North as usize][idx] = Bitboard::new(0x01010101_01010100 << idx);

        rays[Direction::East as usize][idx] = Bitboard::new((1 << idx) - (1 << (idx & 56)));

        rays[Direction::South as usize][idx] = Bitboard::new(0x00808080_80808080 >> (63 - idx));

        rays[Direction::West as usize][idx] = Bitboard::new(2 * ((1 << (idx | 7)) - (1 << idx)));

        let square = Square::from(idx as u8);

        rays[Direction::NorthEast as usize][idx] = shift_east(
            Bitboard::new(0x2040810204080),
            square
                .file
                .checked_shr((7 - square.rank as u32) * 8)
                .unwrap_or(0),
        );

        rays[Direction::SouthEast as usize][idx] = Bitboard::new(0);

        rays[Direction::SouthWest as usize][idx] = Bitboard::new(0);

        rays[Direction::NorthWest as usize][idx] = Bitboard::new(0);
    }

    rays
}

fn shift_east(bb: Bitboard, n: u8) -> Bitboard {
    let mut output = bb;
    for _ in 0..n {
        output = (output << 1) & !(File::A as u64);
    }

    output
}

fn shift_west(bb: Bitboard, n: u8) -> Bitboard {
    let mut output = bb;
    for _ in 0..n {
        output = (output >> 1) & !(File::H as u64);
    }

    output
}
