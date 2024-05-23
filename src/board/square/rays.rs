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

        rays[Direction::East as usize][idx] = Bitboard::new(2 * ((1 << (idx | 7)) - (1 << idx)));

        rays[Direction::South as usize][idx] = Bitboard::new(0x00808080_80808080 >> (63 - idx));

        rays[Direction::West as usize][idx] = Bitboard::new((1 << idx) - (1 << (idx & 56)));

        let square = Square::from(idx as u8);
        dbg!(square);

        rays[Direction::NorthEast as usize][idx] =
            Bitboard::new(0x8040201008040200).shift_east(square.file) << (square.rank as u32 * 8);

        rays[Direction::SouthEast as usize][idx] = Bitboard::new(0x2040810204080)
            .shift_east(square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays[Direction::SouthWest as usize][idx] = Bitboard::new(0x40201008040201)
            .shift_west(7 - square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays[Direction::NorthWest as usize][idx] = Bitboard::new(0x102040810204000)
            .shift_west(7 - square.file)
            << (square.rank as u32 * 8);
    }

    rays
}
