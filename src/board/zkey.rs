use std::hash::Hash;
use std::sync::OnceLock;

use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::board::{Color, Kind};

use super::ply::castling::CastlingKind;
use super::ply::castling::CastlingStatus;
use super::square::Square;
use super::{Board, BoardBuilder};

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
    pub const fn new() -> Self {
        Self {
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
    pub fn init() -> Self {
        assert!(TABLE.get().is_none());
        let mut table = Self::new();

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZKey(u64);

impl Hash for ZKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}

impl nohash_hasher::IsEnabled for ZKey {}

impl From<&Board> for ZKey {
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
    fn from(board: &Board) -> Self {
        let mut key = Self(0);

        for square in 0..64u8 {
            if let Some(piece) = board.get_piece(Square::from(square)) {
                key.0 ^= TABLE.get_or_init(ZTable::init).pieces[usize::from(piece.get_color())]
                    [usize::from(piece)][usize::from(square)];
            }
        }

        if board.castle_status(CastlingKind::WhiteKingside) == CastlingStatus::Available {
            key.0 ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::WhiteKingside)];
        }
        if board.castle_status(CastlingKind::WhiteQueenside) == CastlingStatus::Available {
            key.0 ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::WhiteQueenside)];
        }
        if board.castle_status(CastlingKind::BlackKingside) == CastlingStatus::Available {
            key.0 ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::BlackKingside)];
        }
        if board.castle_status(CastlingKind::BlackQueenside) == CastlingStatus::Available {
            key.0 ^=
                TABLE.get_or_init(ZTable::init).castling[usize::from(CastlingKind::BlackQueenside)];
        }

        if let Some(file) = board.en_passant_file {
            key.0 ^= TABLE.get_or_init(ZTable::init).en_passant[usize::from(file)];
        }

        if board.current_turn == Color::White {
            key.0 ^= TABLE.get_or_init(ZTable::init).white_turn;
        }

        key
    }
}

impl Default for ZKey {
    /// Creates a new Zobrist key with a default value.
    ///
    /// # Returns
    ///
    /// * `ZKey` - The new Zobrist key
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    ///
    /// let zkey = ZKey::default();
    /// ```
    fn default() -> Self {
        Self::from(&BoardBuilder::construct_starting_board().build())
    }
}

impl ZKey {
    /// Creates a new Zobrist key with a default value.
    ///
    /// # Returns
    ///
    /// * `ZKey` - The new Zobrist key
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    ///
    /// let zkey = ZKey::new();
    /// ```
    pub const fn new() -> Self {
        Self(0)
    }

    /// Adds or removes a piece from the Zobrist key.
    /// Note that this function does not check if the piece is already present, so attempting to remove a piece that does not exist will actually add it instead.
    ///
    /// # Arguments
    /// * `piece` - The piece to add or remove
    /// * `square` - The square to add or remove the piece from
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    /// use crate::board::piece::Kind;
    /// use crate::board::square::Square;
    ///
    /// let mut zkey = ZKey::new();
    /// let piece = Kind::Pawn(Color::White);
    /// let square = Square::from(0);
    ///
    /// zkey.add_or_remove_piece(piece, square);
    /// ```
    pub fn add_or_remove_piece(&mut self, piece: Kind, square: Square) {
        self.0 ^= TABLE.get_or_init(ZTable::init).pieces[usize::from(piece.get_color())]
            [usize::from(piece)][usize::from(square)];
    }

    /// Adds or removes castling rights from the Zobrist key.
    /// This function flips the value of castling rights, so performing the same operation twice will revert the change.
    ///
    /// # Arguments
    /// * `castling` - The castling rights to add or remove
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    /// use crate::board::ply::castling::CastlingKind;
    ///
    /// let mut zkey = ZKey::new();
    /// let castling = CastlingKind::WhiteKingside;
    ///
    /// zkey.change_castling_rights(castling);
    /// ```
    pub fn change_castling_rights(&mut self, castling: CastlingKind) {
        self.0 ^= TABLE.get_or_init(ZTable::init).castling[usize::from(castling)];
    }

    /// Changes the en passant file in the Zobrist key.
    /// Note that this function does not unset the previous en passant file, so it is up to the caller to ensure that the correct file is set.
    ///
    /// # Arguments
    /// * `file` - The file to set for en passant
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    /// use crate::board::square::Square;
    ///
    /// let mut zkey = ZKey::new();
    /// let file = Square::from(0).file();
    ///
    /// zkey.change_en_passant(file);
    /// ```
    pub fn change_en_passant(&mut self, file: u8) {
        self.0 ^= TABLE.get_or_init(ZTable::init).en_passant[usize::from(file)];
    }

    /// Changes the turn in the Zobrist key.
    /// This function flips the value of the turn, so performing the same operation twice will revert the change.
    ///
    /// # Example
    /// ```
    /// use crate::board::zkey::ZKey;
    /// use crate::board::Color;
    ///
    /// let mut zkey = ZKey::new();
    /// zkey.change_turn();
    /// ```
    pub fn change_turn(&mut self) {
        self.0 ^= TABLE.get_or_init(ZTable::init).white_turn; // Black's turn is implied by not being White's turn
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::board::Ply;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn test_ztable_unique() {
        let table = TABLE.get_or_init(ZTable::init);
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

        assert_eq!(zkey.0, 0);
    }

    #[test]
    fn test_zkey_from_board_startpos() {
        let zkey = ZKey::from(&Board::default());
        const START_POS_KEY: u64 = 8891004743231992090; // Current start position Zobrist key using the random seed

        assert_eq!(zkey.0, START_POS_KEY);
    }

    #[test]
    fn test_zkey_different_fen_different_hash() {
        let zkey0 = ZKey::from(&Board::from_fen(
            "2rq1rk1/1b2bp2/p3pn1Q/1p2N3/3P4/2NB4/PP3PPP/R5K1 w - - 1 20",
        ));
        let zkey1 = ZKey::from(&Board::from_fen(
            "rnbq1rk1/1p2bppp/p3pn2/4N3/3P4/2NB4/PP3PPP/R1BQK2R b KQ - 1 10",
        ));

        assert_ne!(zkey0.0, 0);
        assert_ne!(zkey1.0, 0);
        assert_ne!(zkey0, zkey1);
    }

    #[test]
    fn test_zkey_eq_empty() {
        let zkey = ZKey::new();
        let other = ZKey::new();

        assert_eq!(zkey, other);
    }

    #[test]
    fn test_zkey_add_or_remove_piece() {
        let mut board = Board::default();
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        assert_eq!(zkey.0, 8891004743231992090); // Current start position Zobrist key using the random seed

        let piece = Kind::Pawn(Color::White);
        let from_sq = Square::from("a2");
        let to_sq = Square::from("a3"); // no en passant

        zkey.add_or_remove_piece(piece, from_sq);
        zkey.add_or_remove_piece(piece, to_sq);
        zkey.change_turn();

        board.make_move(Ply::new(from_sq, to_sq, piece));
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_en_passant() {
        let mut board = Board::default();
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        assert_eq!(zkey.0, 8891004743231992090); // Current start position Zobrist key using the random seed

        let piece = Kind::Pawn(Color::White);
        let from_sq = Square::from("a2");
        let to_sq = Square::from("a4"); // en passant has changed
        let ply = Ply::builder(from_sq, to_sq, piece)
            .double_pawn_push(true)
            .build();

        zkey.add_or_remove_piece(piece, from_sq);
        zkey.add_or_remove_piece(piece, to_sq);
        zkey.change_en_passant(to_sq.file);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_en_passant_reset() {
        let mut board = Board::default();
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        assert_eq!(zkey.0, 8891004743231992090); // Current start position Zobrist key using the random seed

        let piece = Kind::Pawn(Color::White);
        let from_sq = Square::from("a2");
        let to_sq = Square::from("a4"); // en passant has changed
        let ply = Ply::builder(from_sq, to_sq, piece)
            .double_pawn_push(true)
            .build();

        zkey.add_or_remove_piece(piece, from_sq);
        zkey.add_or_remove_piece(piece, to_sq);
        zkey.change_en_passant(to_sq.file);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        zkey.change_en_passant(to_sq.file); // Undo en passant`

        let piece = Kind::Knight(Color::Black);
        let from_sq = Square::from("b8");
        let to_sq = Square::from("c5");
        let ply = Ply::builder(from_sq, to_sq, piece).build();

        zkey.add_or_remove_piece(piece, from_sq);
        zkey.add_or_remove_piece(piece, to_sq);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_capture() {
        let mut board = Board::from_fen("8/6K1/8/4qP2/6k1/8/8/8 b - - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let moving_piece = Kind::Queen(Color::Black);
        let captured_piece = Kind::Pawn(Color::White);
        let from_sq = Square::from("e5");
        let to_sq = Square::from("f5");
        let ply = Ply::builder(from_sq, to_sq, moving_piece)
            .captured(captured_piece)
            .build();

        zkey.add_or_remove_piece(moving_piece, from_sq);
        zkey.add_or_remove_piece(moving_piece, to_sq);
        zkey.add_or_remove_piece(captured_piece, to_sq);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_en_passant_capture() {
        let mut board = Board::from_fen("6k1/8/8/8/5Pp1/8/8/6K1 b - f3 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let moving_piece = Kind::Pawn(Color::Black);
        let captured_piece = Kind::Pawn(Color::White);
        let from_sq = Square::from("g4");
        let to_sq = Square::from("f3");
        let en_passant_sq = Square::from("f4");
        let ply = Ply::builder(from_sq, to_sq, moving_piece)
            .captured(captured_piece)
            .en_passant(true)
            .build();

        zkey.add_or_remove_piece(moving_piece, from_sq);
        zkey.add_or_remove_piece(moving_piece, to_sq);
        zkey.add_or_remove_piece(captured_piece, en_passant_sq);
        zkey.change_en_passant(en_passant_sq.file);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_promotion() {
        let mut board = Board::from_fen("k7/5P2/8/8/8/8/8/6K1 w - - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let moving_piece = Kind::Pawn(Color::White);
        let promoted_piece = Kind::Queen(Color::White);
        let from_sq = Square::from("f7");
        let to_sq = Square::from("f8");
        let ply = Ply::builder(from_sq, to_sq, moving_piece)
            .promoted_to(promoted_piece)
            .build();

        zkey.add_or_remove_piece(moving_piece, from_sq);
        zkey.add_or_remove_piece(promoted_piece, to_sq);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_add_or_remove_piece_capture_promotion() {
        let mut board = Board::from_fen("k5q1/5P2/8/8/8/8/8/6K1 w - - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let moving_piece = Kind::Pawn(Color::White);
        let promoted_piece = Kind::Queen(Color::White);
        let captured_piece = Kind::Queen(Color::Black);
        let from_sq = Square::from("f7");
        let to_sq = Square::from("g8");
        let ply = Ply::builder(from_sq, to_sq, moving_piece)
            .captured(captured_piece)
            .promoted_to(promoted_piece)
            .build();

        zkey.add_or_remove_piece(moving_piece, from_sq);
        zkey.add_or_remove_piece(promoted_piece, to_sq);
        zkey.add_or_remove_piece(captured_piece, to_sq);
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_change_castling_rights_white_kingside() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let ply = Ply::builder(
            Square::from("e1"),
            Square::from("g1"),
            Kind::King(Color::White),
        )
        .castles(true)
        .build();

        zkey.change_castling_rights(CastlingKind::WhiteKingside);
        zkey.change_castling_rights(CastlingKind::WhiteQueenside); // Both kingside and queenside rights are removed
        zkey.add_or_remove_piece(Kind::King(Color::White), Square::from("e1"));
        zkey.add_or_remove_piece(Kind::Rook(Color::White), Square::from("h1"));
        zkey.add_or_remove_piece(Kind::King(Color::White), Square::from("g1"));
        zkey.add_or_remove_piece(Kind::Rook(Color::White), Square::from("f1"));
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_change_castling_rights_white_queenside() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let ply = Ply::builder(
            Square::from("e1"),
            Square::from("c1"),
            Kind::King(Color::White),
        )
        .castles(true)
        .build();

        zkey.change_castling_rights(CastlingKind::WhiteKingside);
        zkey.change_castling_rights(CastlingKind::WhiteQueenside); // Both kingside and queenside rights are removed
        zkey.add_or_remove_piece(Kind::King(Color::White), Square::from("e1"));
        zkey.add_or_remove_piece(Kind::Rook(Color::White), Square::from("a1"));
        zkey.add_or_remove_piece(Kind::King(Color::White), Square::from("c1"));
        zkey.add_or_remove_piece(Kind::Rook(Color::White), Square::from("d1"));
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_change_castling_rights_black_kingside() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let ply = Ply::builder(
            Square::from("e8"),
            Square::from("g8"),
            Kind::King(Color::Black),
        )
        .castles(true)
        .build();

        zkey.change_castling_rights(CastlingKind::BlackKingside);
        zkey.change_castling_rights(CastlingKind::BlackQueenside); // Both kingside and queenside rights are removed
        zkey.add_or_remove_piece(Kind::King(Color::Black), Square::from("e8"));
        zkey.add_or_remove_piece(Kind::Rook(Color::Black), Square::from("h8"));
        zkey.add_or_remove_piece(Kind::King(Color::Black), Square::from("g8"));
        zkey.add_or_remove_piece(Kind::Rook(Color::Black), Square::from("f8"));
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }

    #[test]
    fn test_zkey_change_castling_rights_black_queenside() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
        let original_zkey = ZKey::from(&board);
        let mut zkey = original_zkey.clone();

        let ply = Ply::builder(
            Square::from("e8"),
            Square::from("c8"),
            Kind::King(Color::Black),
        )
        .castles(true)
        .build();

        zkey.change_castling_rights(CastlingKind::BlackKingside);
        zkey.change_castling_rights(CastlingKind::BlackQueenside); // Both kingside and queenside rights are removed
        zkey.add_or_remove_piece(Kind::King(Color::Black), Square::from("e8"));
        zkey.add_or_remove_piece(Kind::Rook(Color::Black), Square::from("a8"));
        zkey.add_or_remove_piece(Kind::King(Color::Black), Square::from("c8"));
        zkey.add_or_remove_piece(Kind::Rook(Color::Black), Square::from("d8"));
        zkey.change_turn();

        board.make_move(ply);
        assert_eq!(zkey, ZKey::from(&board));
        assert_eq!(zkey, board.zkey);
        assert_eq!(board.zkey, ZKey::from(&board));

        board.unmake_move();
        assert_eq!(board.zkey, original_zkey);
    }
}
