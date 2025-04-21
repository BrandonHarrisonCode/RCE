use super::square::Square;
use std::fmt;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Deref, Mul, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub,
};

macro_rules! bitboard_operator {
    ( $($type:ty, $trait:ident, $fn:ident;)* ) => {$(
        impl $trait<$type> for Bitboard {
            type Output = Self;

            fn $fn(self, rhs: $type) -> Self::Output {
                Self($trait::$fn(self.0, rhs))
            }
        }
    )*};

    ( $($trait:ident, $fn:ident;)* ) => {$(
        impl $trait for Bitboard {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self::Output {
                Self($trait::$fn(self.0, rhs.0))
            }
        }
    )*};
}

macro_rules! bitboard_assignment_operator {
    ( $($type:ty, $trait:ident, $fn:ident;)* ) => {$(
        impl $trait<$type> for Bitboard {
            fn $fn(&mut self, rhs: $type) {
                $trait::<$type>::$fn(&mut self.0, rhs)
            }
        }
    )*};

    ( $($trait:ident, $fn:ident;)* ) => {$(
        impl $trait for Bitboard {
            fn $fn(&mut self, rhs: Self) {
                $trait::$fn(self, rhs.0)
            }
        }
    )*};
}

bitboard_operator! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}
bitboard_operator! {
    u64, BitAnd, bitand;
    u64, BitOr, bitor;
    u64, BitXor, bitxor;
    usize, Shr, shr;
}

bitboard_assignment_operator! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
}
bitboard_assignment_operator! {
    u64, BitAndAssign, bitand_assign;
    u64, BitOrAssign, bitor_assign;
    usize, ShrAssign, shr_assign;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Bitboard(u64);

#[repr(u64)]
#[allow(dead_code)]
pub enum Rank {
    First = 0x0000_0000_0000_00ff,
    Second = 0x0000_0000_0000_ff00,
    Third = 0x0000_0000_00ff_0000,
    Fourth = 0x0000_0000_ff00_0000,
    Fifth = 0x0000_00ff_0000_0000,
    Sixth = 0x0000_ff00_0000_0000,
    Seventh = 0x00ff_0000_0000_0000,
    Eighth = 0xff00_0000_0000_0000,
}

#[repr(u64)]
#[allow(dead_code)]
pub enum File {
    A = 0x0101_0101_0101_0101,
    B = 0x0202_0202_0202_0202,
    C = 0x0404_0404_0404_0404,
    D = 0x0808_0808_0808_0808,
    E = 0x1010_1010_1010_1010,
    F = 0x2020_2020_2020_2020,
    G = 0x4040_4040_4040_4040,
    H = 0x8080_8080_8080_8080,
}

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Bitboard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_add(rhs.0).unwrap_or(0))
    }
}

impl Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Mul for Bitboard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

impl Add<u64> for Bitboard {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0.checked_add(rhs).unwrap_or(0))
    }
}

impl Sub<u64> for Bitboard {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0.saturating_sub(rhs))
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

impl ShlAssign<u32> for Bitboard {
    fn shl_assign(&mut self, rhs: u32) {
        self.0 = self.0.checked_shl(rhs).unwrap_or(0);
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[allow(clippy::cast_possible_truncation)]
impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mask = 0xFF;

        writeln!(f, "Debug bitboard: {:0>64b}", self.0)?;
        for i in (0..8).rev() {
            writeln!(
                f,
                "{:0>8b}",
                (((self.0 >> (8 * i)) & mask) as u8).reverse_bits()
            )?;
        }

        Ok(())
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Self(1 << u8::from(square))
    }
}

impl From<usize> for Bitboard {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

#[allow(clippy::cast_possible_truncation)]
impl From<Bitboard> for usize {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0 as Self
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

#[allow(clippy::cast_possible_truncation)]
impl From<Bitboard> for Vec<Square> {
    fn from(bitboard: Bitboard) -> Self {
        let mut squares = vec![];

        let mut mask = bitboard.0;
        while mask != 0 {
            let idx = mask.trailing_zeros() as u8;
            squares.push(Square::from(idx));
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

    /// Moves the entire bitboard `n` squares East
    pub fn shift_east(self, n: u8) -> Self {
        let mut output = self;
        for _ in 0..n {
            output = (output << 1) & !(File::A as u64);
        }

        output
    }

    /// Moves the entire bitboard `n` squares West
    pub fn shift_west(self, n: u8) -> Self {
        let mut output = self;
        for _ in 0..n {
            output = (output >> 1) & !(File::H as u64);
        }

        output
    }

    /// Safe wrapper around the `count_ones` intrinsic
    pub const fn count_ones(self) -> u32 {
        unsafe { self.count_ones_helper() }
    }

    /// `count_ones` using the built-in `x86_64` `popcnt` instruction
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    const unsafe fn count_ones_helper(self) -> u32 {
        self.0.count_ones()
    }

    /// Finds the index of the least significant bit and set it to 0
    pub const fn drop_forward(&mut self) -> u32 {
        let idx = self.bitscan_forward();
        self.0 &= self.0 - 1;
        idx
    }

    /// Safe wrapper around the `bitscan_forward` intrinsic
    pub const fn bitscan_forward(self) -> u32 {
        unsafe { self.bitscan_forward_helper() }
    }

    /// `trailing_zeros` using the built-in `x86_64` `bmi1` instruction
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "bmi1"))]
    const unsafe fn bitscan_forward_helper(self) -> u32 {
        self.0.trailing_zeros()
    }

    /// Safe wrapper around the `bitscan_reverse` intrinsic
    pub const fn bitscan_reverse(self) -> u32 {
        unsafe { self.bitscan_reverse_helper() }
    }

    /// `leading_zeros` using the built-in `x86_64` `bmi1` instruction
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "bmi1"))]
    const unsafe fn bitscan_reverse_helper(self) -> u32 {
        63 - self.0.leading_zeros()
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter::new(self)
    }
}

pub struct BitboardIter {
    bitboard: Bitboard,
}

impl BitboardIter {
    const fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    #[allow(clippy::cast_possible_truncation)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.is_empty() {
            return None;
        }

        Some(Square::from(self.bitboard.drop_forward() as u8))
    }
}
