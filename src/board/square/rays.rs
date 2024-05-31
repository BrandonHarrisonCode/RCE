use super::Direction;
use super::Square;
use std::sync::OnceLock;

use crate::board::bitboard::Bitboard;

pub struct Rays {
    pub rays: [[Bitboard; 8]; 64],
}

impl Rays {
    pub fn new() -> Self {
        Self { rays: init_rays() }
    }
}

pub static RAYS: OnceLock<Rays> = OnceLock::new();

fn init_rays() -> [[Bitboard; 8]; 64] {
    let mut rays = [[Bitboard::new(0); 8]; 64];

    for idx in 0..64 {
        rays[idx][Direction::North as usize] = Bitboard::new(0x01010101_01010100 << idx);

        rays[idx][Direction::East as usize] = Bitboard::new(2 * ((1 << (idx | 7)) - (1 << idx)));

        rays[idx][Direction::South as usize] = Bitboard::new(0x00808080_80808080 >> (63 - idx));

        rays[idx][Direction::West as usize] = Bitboard::new((1 << idx) - (1 << (idx & 56)));

        let square = Square::from(idx as u8);

        rays[idx][Direction::NorthEast as usize] =
            Bitboard::new(0x8040201008040200).shift_east(square.file) << (square.rank as u32 * 8);

        rays[idx][Direction::SouthEast as usize] = Bitboard::new(0x2040810204080)
            .shift_east(square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays[idx][Direction::SouthWest as usize] = Bitboard::new(0x40201008040201)
            .shift_west(7 - square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays[idx][Direction::NorthWest as usize] = Bitboard::new(0x102040810204000)
            .shift_west(7 - square.file)
            << (square.rank as u32 * 8);
    }

    rays
}
