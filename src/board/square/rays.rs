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

#[allow(clippy::cast_possible_truncation)]
fn init_rays() -> [[Bitboard; 8]; 64] {
    let mut rays = [[Bitboard::new(0); 8]; 64];

    for (idx, rays_at_square) in rays.iter_mut().enumerate() {
        rays_at_square[Direction::North as usize] = Bitboard::new(0x0101_0101_0101_0100 << idx);

        rays_at_square[Direction::East as usize] =
            Bitboard::new(2 * ((1 << (idx | 7)) - (1 << idx)));

        rays_at_square[Direction::South as usize] =
            Bitboard::new(0x0080_8080_8080_8080 >> (63 - idx));

        rays_at_square[Direction::West as usize] = Bitboard::new((1 << idx) - (1 << (idx & 56)));

        let square = Square::from(idx as u8);

        rays_at_square[Direction::NorthEast as usize] = Bitboard::new(0x8040_2010_0804_0200)
            .shift_east(square.file)
            << (u32::from(square.rank) * 8);

        rays_at_square[Direction::SouthEast as usize] = Bitboard::new(0x2_0408_1020_4080)
            .shift_east(square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays_at_square[Direction::SouthWest as usize] = Bitboard::new(0x40_2010_0804_0201)
            .shift_west(7 - square.file)
            >> ((7 - square.rank) * 8) as usize;

        rays_at_square[Direction::NorthWest as usize] = Bitboard::new(0x102_0408_1020_4000)
            .shift_west(7 - square.file)
            << (u32::from(square.rank) * 8);
    }

    rays
}
