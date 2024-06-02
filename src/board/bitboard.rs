use super::square::Square;
use std::fmt;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Deref, Mul, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Bitboard(u64);

#[repr(u64)]
#[allow(dead_code)]
pub enum Rank {
    First = 0x00000000_000000ff,
    Second = 0x00000000_0000ff00,
    Third = 0x00000000_00ff0000,
    Fourth = 0x00000000_ff000000,
    Fifth = 0x000000ff_00000000,
    Sixth = 0x0000ff00_00000000,
    Seventh = 0x00ff0000_00000000,
    Eighth = 0xff000000_00000000,
}

#[repr(u64)]
#[allow(dead_code)]
pub enum File {
    A = 0x01010101_01010101,
    B = 0x02020202_02020202,
    C = 0x04040404_04040404,
    D = 0x08080808_08080808,
    E = 0x10101010_10101010,
    F = 0x20202020_20202020,
    G = 0x40404040_40404040,
    H = 0x80808080_80808080,
}

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Mul for Bitboard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl Mul<u64> for Bitboard {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0.wrapping_mul(rhs))
    }
}

impl Shl<u32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0.checked_shl(rhs).unwrap_or(0))
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShlAssign<u32> for Bitboard {
    fn shl_assign(&mut self, rhs: u32) {
        self.0 = self.0.checked_shl(rhs).unwrap_or(0);
    }
}

impl ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 = self.0 >> rhs;
    }
}
impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mask = 0xFF;

        write!(f, "Debug bitboard: {:0>64b}\n", self.0)?;
        for i in (0..8).rev() {
            write!(
                f,
                "{:0>8b}\n",
                (((self.0 >> (8 * i)) & mask) as u8).reverse_bits()
            )?;
        }

        Ok(())
    }
}

impl From<usize> for Bitboard {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<Bitboard> for usize {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0 as usize
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}

impl From<Bitboard> for Vec<Square> {
    fn from(bitboard: Bitboard) -> Self {
        let mut squares = vec![];

        let mut mask = bitboard.0;
        while mask != 0 {
            let idx = mask.trailing_zeros();
            squares.push(Square::from(idx as u8));
            mask &= mask - 1;
        }

        squares
    }
}

impl Bitboard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Removes the edges of the bitboard, i.e. the first and eighth ranks and the a and h files
    ///
    /// # Returns
    ///
    /// * `Bitboard` - The bitboard with the edges removed
    ///
    /// # Example
    /// ```
    /// use chess::bitboard::Bitboard;
    ///
    /// let bitboard = Square::from("a1").get_rank_mask();
    /// let trimmed = bitboard.trim_edges();
    /// ```
    pub fn trim_edges(self) -> Self {
        self & !(Rank::First as u64)
            & !(Rank::Eighth as u64)
            & !(File::A as u64)
            & !(File::H as u64)
    }

    pub fn shift_east(self, n: u8) -> Bitboard {
        let mut output = self;
        for _ in 0..n {
            output = (output << 1) & !(File::A as u64);
        }

        output
    }

    pub fn shift_west(self, n: u8) -> Bitboard {
        let mut output = self;
        for _ in 0..n {
            output = (output >> 1) & !(File::H as u64);
        }

        output
    }

    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    pub fn drop_forward(&mut self) -> u32 {
        let idx = self.bitscan_forward();
        self.0 &= self.0 - 1;
        idx
    }

    pub fn bitscan_forward(self) -> u32 {
        assert_ne!(self.0, 0);
        self.0.trailing_zeros()
    }

    pub fn bitscan_reverse(self) -> u32 {
        63 - self.0.leading_zeros()
    }
}
