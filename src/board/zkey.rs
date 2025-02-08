use std::sync::OnceLock;

use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::board::Color;

use super::ply::castling::CastlingKind;
use super::ply::castling::CastlingStatus;
use super::square::Square;
use super::Board;

const SEED: u64 = 0xBEEF_CAFE;

/// A Zobrist hash table with random numbers for unique identities.
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
#[derive(Debug, Clone, Copy)]
pub struct ZKey {
    key: u64,
    white_kingside: CastlingStatus,
    white_queenside: CastlingStatus,
    black_kingside: CastlingStatus,
    black_queenside: CastlingStatus,
    en_passant: Option<u8>,
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
        let mut key = ZKey::new();

        for square in 0..64u8 {
            if let Some(piece) = board.get_piece(Square::from(square)) {
                key.key ^= TABLE.get_or_init(ZTable::init).pieces[usize::from(piece.get_color())]
                    [usize::from(piece)][usize::from(square)];
            }
        }

        if board.castle_status(CastlingKind::WhiteKingside) == CastlingStatus::Available {
            key.key ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::WhiteKingside)];
            key.white_kingside = CastlingStatus::Available;
        }
        if board.castle_status(CastlingKind::WhiteQueenside) == CastlingStatus::Available {
            key.key ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::WhiteQueenside)];
            key.white_queenside = CastlingStatus::Available;
        }
        if board.castle_status(CastlingKind::BlackKingside) == CastlingStatus::Available {
            key.key ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::BlackKingside)];
            key.black_kingside = CastlingStatus::Available;
        }
        if board.castle_status(CastlingKind::BlackQueenside) == CastlingStatus::Available {
            key.key ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::BlackQueenside)];
            key.black_queenside = CastlingStatus::Available;
        }

        if let Some(file) = board.en_passant_file {
            key.key ^= TABLE.get_or_init(ZTable::init).en_passant[usize::from(file)];
            key.en_passant = Some(file);
        }

        if board.current_turn == Color::White {
            key.key ^= TABLE.get_or_init(ZTable::init).white_turn;
        }

        key
    }
}

impl PartialEq for ZKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for ZKey {}

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
            white_kingside: CastlingStatus::Unavailable,
            white_queenside: CastlingStatus::Unavailable,
            black_kingside: CastlingStatus::Unavailable,
            black_queenside: CastlingStatus::Unavailable,
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

    #[test]
    fn test_ztable_new() {
        let table = ZTable::new();

        for square in 0..64usize {
            for piece in 0..6usize {
                assert_eq!(table.pieces[Color::White as usize][piece][square], 0);
                assert_eq!(table.pieces[Color::Black as usize][piece][square], 0);
            }
        }

        for i in 0..4usize {
            assert_eq!(table.castling[i], 0);
        }

        for i in 0..8usize {
            assert_eq!(table.en_passant[i], 0);
        }

        assert_eq!(table.white_turn, 0);
    }

    #[test]
    fn test_zkey_new() {
        let zkey = ZKey::new();

        assert_eq!(zkey.key, 0);
        assert_eq!(zkey.white_kingside, CastlingStatus::Unavailable);
        assert_eq!(zkey.white_queenside, CastlingStatus::Unavailable);
        assert_eq!(zkey.black_kingside, CastlingStatus::Unavailable);
        assert_eq!(zkey.black_queenside, CastlingStatus::Unavailable);
        assert!(zkey.en_passant.is_none());
    }

    #[test]
    fn test_zkey_from_board_startpos() {
        let zkey = ZKey::from(Board::default());
        const START_POS_KEY: u64 = 8891004743231992090; // Current start position Zobrist key using the random seed

        assert_eq!(zkey.key, START_POS_KEY);
        assert_eq!(zkey.white_kingside, CastlingStatus::Available);
        assert_eq!(zkey.white_queenside, CastlingStatus::Available);
        assert_eq!(zkey.black_kingside, CastlingStatus::Available);
        assert_eq!(zkey.black_queenside, CastlingStatus::Available);
        assert!(zkey.en_passant.is_none());
    }

    #[test]
    fn test_zkey_different_fen_different_hash() {
        let zkey0 = ZKey::from(Board::from_fen(
            "2rq1rk1/1b2bp2/p3pn1Q/1p2N3/3P4/2NB4/PP3PPP/R5K1 w - - 1 20",
        ));
        let zkey1 = ZKey::from(Board::from_fen(
            "rnbq1rk1/1p2bppp/p3pn2/4N3/3P4/2NB4/PP3PPP/R1BQK2R b KQ - 1 10",
        ));

        assert_ne!(zkey0.key, 0);
        assert_ne!(zkey1.key, 0);
        assert_ne!(zkey0, zkey1);
    }

    #[test]
    fn test_zkey_eq_empty() {
        let zkey = ZKey::new();
        let other = ZKey::new();

        assert_eq!(zkey, other);
    }

    #[test]
    fn test_zkey_eq_check_key_only() {
        let zkey = ZKey {
            key: 123,
            ..ZKey::new()
        };
        let same = ZKey {
            key: 123,
            white_kingside: CastlingStatus::Available,
            ..ZKey::new()
        };
        let different = ZKey {
            key: 321,
            ..ZKey::new()
        };

        assert_eq!(zkey, same);
        assert_ne!(zkey, different);
    }
}
