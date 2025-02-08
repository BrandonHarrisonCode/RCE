use std::sync::OnceLock;

use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::board::Color;

use super::bitboard::File;
use super::ply::castling::CastlingStatus;
use super::Board;

const SEED: u64 = 0xBEEF_CAFE;

/// A Zobrist hash table with random numbers for each entry.
pub struct ZTable {
    pieces: [[[u64; 64]; 6]; 2], // 64 squares, 6 pieces, 2 colors
    castling: [u64; 4],
    en_passant: [u64; 8],
    white_turn: u64,
}

static TABLE: OnceLock<ZTable> = OnceLock::new();

impl ZTable {
    /// Creates an empty Zobrist hash table.
    ///
    /// # Returns
    ///
    /// * `ZTable` - The empty Zobrist hash table
    ///
    /// # Example
    /// ```
    /// use chess::board::zkey::ZTable;
    ///
    /// let table = ZTable::new();
    /// ```
    pub fn new() -> ZTable {
        ZTable {
            pieces: [[[0; 64]; 6]; 2],
            castling: [0; 4],
            en_passant: [0; 8],
            white_turn: 0,
        }
    }

    /// Initializes the Zobrist hash table with random numbers.
    ///
    /// # Returns
    ///
    /// * `ZTable` - The initialized Zobrist hash table
    ///
    /// # Example
    /// ```
    /// use chess::board::zkey::ZTable;
    ///
    /// let table = ZTable::init();
    /// ```
    pub fn init() -> ZTable {
        assert!(TABLE.get().is_none());
        let mut table = ZTable::new();

        let mut rng = ChaCha8Rng::seed_from_u64(SEED);

        for square in 0..64usize {
            for piece in 0..6usize {
                table.pieces[Color::White as usize][piece][square] = rng.next_u64();
                table.pieces[Color::Black as usize][piece][square] = rng.next_u64();
            }
        }

        for i in 0..4usize {
            table.castling[i] = rng.next_u64();
        }

        for i in 0..8usize {
            table.en_passant[i] = rng.next_u64();
        }

        table.white_turn = rng.next_u64();

        table
    }
}

/// A Zobrist key for a board position.
pub struct ZKey {
    key: u64,
    white_kingside: CastlingStatus,
    white_queenside: CastlingStatus,
    black_kingside: CastlingStatus,
    black_queenside: CastlingStatus,
    en_passant: Option<File>,
}

impl From<Board> for ZKey {
    /// Converts a board position to a Zobrist key.
    ///
    /// # Arguments
    ///
    /// * `board` - The board position
    ///
    /// # Returns
    ///
    /// * `ZKey` - The Zobrist key for the board position
    ///
    /// # Example
    /// ```
    /// use chess::board::Board;
    /// use chess::board::zkey::ZKey;
    ///
    /// let board = Board::start_pos();
    /// let zkey = ZKey::from(board);
    /// ```
    fn from(board: Board) -> Self {
        unimplemented!()
    }
}

impl ZKey {
    /// Creates a new Zobrist key.
    ///
    /// # Returns
    ///
    /// * `ZKey` - The new Zobrist key
    ///
    /// # Example
    /// ```
    /// use chess::board::zkey::ZKey;
    ///
    /// let zkey = ZKey::new();
    /// ```
    pub const fn new() -> ZKey {
        ZKey {
            key: 0,
            white_kingside: CastlingStatus::Unavailiable,
            white_queenside: CastlingStatus::Unavailiable,
            black_kingside: CastlingStatus::Unavailiable,
            black_queenside: CastlingStatus::Unavailiable,
            en_passant: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn test_ztable_unique() {
        let table = ZTable::init();
        let mut map = HashSet::new();

        for square in 0..64usize {
            for piece in 0..6usize {
                assert_eq!(
                    map.insert(table.pieces[Color::White as usize][piece][square]),
                    true,
                    "Duplicate hash in ZTable!"
                );
                assert_eq!(
                    map.insert(table.pieces[Color::Black as usize][piece][square]),
                    true,
                    "Duplicate hash in ZTable!"
                );
            }
        }

        for i in 0..4usize {
            assert_eq!(
                map.insert(table.castling[i]),
                true,
                "Duplicate hash in ZTable!"
            );
        }

        for i in 0..8usize {
            assert_eq!(
                map.insert(table.en_passant[i]),
                true,
                "Duplicate hash in ZTable!"
            );
        }

        assert_eq!(
            map.insert(table.white_turn),
            true,
            "Duplicate hash in ZTable!"
        );
    }
}
