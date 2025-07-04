use rustc_hash::FxHashSet;

use super::piece::Color;
use super::piece::Kind as PieceKind;
use super::ply::castling::CastlingKind;
use super::ply::Ply;
use super::zkey::ZKey;
use super::Board;
use super::CastlingStatus;
use super::Square;

use super::piece_bitboards;
use super::piece_bitboards::builder::Builder as PieceBitboardsBuilder;

#[derive(Default, Clone)]
pub struct BoardBuilder {
    pub current_turn: Color,
    pub halfmove_clock: u16,
    pub fullmove_counter: u16,

    pub en_passant_file: Option<u8>,

    pub bitboards: PieceBitboardsBuilder,

    pub history: Vec<Ply>,
    pub position_history: Vec<ZKey>,
}

impl BoardBuilder {
    /// Creates a new board object that represents the starting board state in a normal game
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board().build();
    /// ```
    pub fn construct_starting_board() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    /// Creates a new board object without any pieces on the board
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_empty_board();
    /// ```
    pub fn construct_empty_board() -> Self {
        Self::default().clear()
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            current_turn: Color::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,

            en_passant_file: None,

            bitboards: PieceBitboardsBuilder::default(),

            history: vec![Ply::default()],
            position_history: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            current_turn: Color::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,

            en_passant_file: None,

            bitboards: PieceBitboardsBuilder::new(),

            history: vec![Ply::default()],
            position_history: vec![],
        }
    }

    pub fn get_last_history(&mut self) -> &mut Ply {
        if self.history.is_empty() {
            self.history.push(Ply::default());
        }
        self.history
            .last_mut()
            .expect("History could not be written to")
    }

    pub const fn clear(mut self) -> Self {
        self.bitboards = piece_bitboards::builder::Builder::new();
        self
    }

    /// Set the color of the player who is currently playing
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the player who is currently playing
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use chess::board::BoardBuilder;
    /// use chess::piece::Color;
    ///
    /// let builder = BoardBuilder::default().white_turn(false);
    /// ```
    pub const fn turn(mut self, color: Color) -> Self {
        self.current_turn = color;
        self
    }

    /// Set the castling status for the specified `CastlingKind`
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of castling being set
    ///
    /// * `value` - The status of the castling being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().castling(Castling::WhiteKingside, CastlingStatus::Unavailiable);
    ///
    /// ```
    pub fn castling(mut self, kind: CastlingKind, value: CastlingStatus) -> Self {
        match kind {
            CastlingKind::WhiteKingside => {
                self.get_last_history().castling_rights.white_kingside = value;
            }
            CastlingKind::WhiteQueenside => {
                self.get_last_history().castling_rights.white_queenside = value;
            }
            CastlingKind::BlackKingside => {
                self.get_last_history().castling_rights.black_kingside = value;
            }
            CastlingKind::BlackQueenside => {
                self.get_last_history().castling_rights.black_queenside = value;
            }
        }
        self
    }

    /// Adds a piece on the specified square
    ///
    /// # Arguments
    ///
    /// * `square` - The square to place the piece on
    ///
    /// * `kind` - The kind of piece to place on the square
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    /// use crate::piece::PieceKind;
    /// use crate::square::Square;
    ///
    /// let builder = BoardBuilder::default().piece(Square::from("a1"), PieceKind::WhiteKing);
    /// ```
    #[allow(dead_code)]
    pub const fn piece(mut self, square: Square, kind: PieceKind) -> Self {
        self.bitboards.add_piece(square, kind);
        self
    }

    /// Set the pawn bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the pawns being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().pawns(Color::Black, 0);
    /// ```
    pub const fn pawns(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.pawns(color, value);
        self
    }

    /// Set the king bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the king being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().king(Color::Black, 0);
    /// ```
    pub const fn king(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.king(color, value);
        self
    }

    /// Set the queen bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the queens being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().queens(Color::Black, 0);
    /// ```
    pub const fn queens(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.queens(color, value);
        self
    }

    /// Set the rook bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the rooks being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().rooks(Color::Black, 0);
    /// ```
    pub const fn rooks(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.rooks(color, value);
        self
    }

    /// Set the bishop bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the bishops being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().bishops(Color::Black, 0);
    /// ```
    pub const fn bishops(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.bishops(color, value);
        self
    }

    /// Set the knight bitmap for the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the knights being set
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().knights(Color::Black, 0);
    /// ```
    pub const fn knights(mut self, color: Color, value: u64) -> Self {
        self.bitboards = self.bitboards.knights(color, value);
        self
    }

    /// Set the history of the board
    ///
    /// # Arguments
    ///
    /// * `history` - The history of the board
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().history(Vec::new());
    /// ```
    pub fn history(mut self, history: &[Ply]) -> Self {
        let previous_last_history_castling_rights = self.get_last_history().castling_rights;
        self.history = history.to_vec();
        self.get_last_history().castling_rights = previous_last_history_castling_rights;
        self
    }

    /// Set the history of the different positions of the board
    ///
    /// # Arguments
    ///
    /// * `position_history` - The position history of the board
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().position_history(Vec::new());
    /// ```
    pub fn position_history(mut self, position_history: &[ZKey]) -> Self {
        self.position_history = position_history.to_vec();
        self
    }

    /// Set the en passant capture file
    ///
    /// # Arguments
    ///
    /// * `en_passant_file` - The file that is availiable for en passant capturing
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().en_passant_file(Some(2));
    /// ```
    pub const fn en_passant_file(mut self, en_passant_file: Option<u8>) -> Self {
        self.en_passant_file = en_passant_file;
        self
    }

    /// Set the halfmove clock of the board
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the halfmove clock
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().halfmove_clock(5);
    /// ```
    pub const fn halfmove_clock(mut self, value: u16) -> Self {
        self.halfmove_clock = value;
        self
    }

    /// Set the fullmove counter of the board
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the fullmove counter
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let builder = BoardBuilder::default().fullmove_counter(5);
    /// ```
    pub const fn fullmove_counter(mut self, value: u16) -> Self {
        self.fullmove_counter = value;
        self
    }

    /// Consume the `BoardBuilder` to create a `Board`
    ///
    /// # Returns
    ///
    /// * `Board` - The board represented by the builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{BoardBuilder, Color, Castling};
    ///
    /// let board: Board = BoardBuilder::default().fullmove_counter(5).build();
    /// ```
    pub fn build(&mut self) -> Board {
        // Ensure that no piece is on the same square as another piece
        assert_eq!(
            self.bitboards.white_bishops
                & self.bitboards.white_knights
                & self.bitboards.white_queens
                & self.bitboards.white_rooks
                & self.bitboards.white_king
                & self.bitboards.white_pawns,
            0
        );
        assert_eq!(
            self.bitboards.black_bishops
                & self.bitboards.black_knights
                & self.bitboards.black_queens
                & self.bitboards.black_rooks
                & self.bitboards.black_king
                & self.bitboards.black_pawns,
            0
        );

        self.history[0].halfmove_clock = self.halfmove_clock;
        let mut output = Board {
            current_turn: self.current_turn,
            fullmove_counter: self.fullmove_counter,
            en_passant_file: self.en_passant_file,
            history: self.history.clone(),
            bitboards: self.bitboards.build(),

            position_history: FxHashSet::default(),
            zkey: ZKey::new(),
        };
        output.zkey = ZKey::from(&output);

        output
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::bitboard::Bitboard;
    use super::super::piece::{Color, Kind};
    use super::super::piece_bitboards::PieceBitboards;
    use super::super::square::Square;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn board_builder_default() {
        let board = BoardBuilder::default().build();
        let correct = BoardBuilder::construct_starting_board().build();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_turn() {
        let board = BoardBuilder::new().turn(Color::Black).build();
        let mut correct = Board {
            current_turn: Color::Black,
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_turn() {
        let board = BoardBuilder::default()
            .turn(Color::Black)
            .turn(Color::White)
            .build();
        let correct = BoardBuilder::construct_starting_board().build();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_kingside_castling() {
        let board = BoardBuilder::default()
            .castling(CastlingKind::WhiteKingside, CastlingStatus::Unavailable)
            .build();

        assert_eq!(
            board
                .history
                .last()
                .expect("No history")
                .castling_rights
                .white_kingside,
            CastlingStatus::Unavailable
        )
    }

    #[test]
    fn board_builder_black_kingside_castling() {
        let board = BoardBuilder::default()
            .castling(CastlingKind::BlackKingside, CastlingStatus::Unavailable)
            .build();

        assert_eq!(
            board
                .history
                .last()
                .expect("No history")
                .castling_rights
                .black_kingside,
            CastlingStatus::Unavailable
        )
    }

    #[test]
    fn board_builder_white_queenside_castling() {
        let board = BoardBuilder::default()
            .castling(CastlingKind::WhiteQueenside, CastlingStatus::Unavailable)
            .build();

        assert_eq!(
            board
                .history
                .last()
                .expect("No history")
                .castling_rights
                .white_queenside,
            CastlingStatus::Unavailable
        )
    }

    #[test]
    fn board_builder_black_queenside_castling() {
        let board = BoardBuilder::default()
            .castling(CastlingKind::BlackQueenside, CastlingStatus::Unavailable)
            .build();

        assert_eq!(
            board
                .history
                .last()
                .expect("No history")
                .castling_rights
                .black_queenside,
            CastlingStatus::Unavailable
        )
    }

    #[test]
    fn board_builder_pawns() {
        let board = BoardBuilder::new()
            .pawns(Color::White, 1)
            .pawns(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_pawns: Bitboard::new(1),
                black_pawns: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_king() {
        let board = BoardBuilder::new()
            .king(Color::White, 1)
            .king(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_king: Bitboard::new(1),
                black_king: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_queens() {
        let board = BoardBuilder::new()
            .queens(Color::White, 1)
            .queens(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_queens: Bitboard::new(1),
                black_queens: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_rooks() {
        let board = BoardBuilder::new()
            .rooks(Color::White, 1)
            .rooks(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_rooks: Bitboard::new(1),
                black_rooks: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_bishops() {
        let board = BoardBuilder::new()
            .bishops(Color::White, 1)
            .bishops(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_bishops: Bitboard::new(1),
                black_bishops: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_knights() {
        let board = BoardBuilder::new()
            .knights(Color::White, 1)
            .knights(Color::Black, 2)
            .build();
        let mut correct = Board {
            bitboards: PieceBitboards {
                white_knights: Bitboard::new(1),
                black_knights: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..BoardBuilder::construct_empty_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_history() {
        let history = vec![Ply::new(
            Square::from("a1"),
            Square::from("a2"),
            Kind::Pawn(Color::White),
        )];
        let board = BoardBuilder::default().history(&history).build();
        let correct = Board {
            history,
            ..BoardBuilder::construct_starting_board().build()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_en_passant() {
        let board = BoardBuilder::default().en_passant_file(Some(1)).build();
        let mut correct = Board {
            en_passant_file: Some(1),
            ..BoardBuilder::construct_starting_board().build()
        };
        correct.zkey = ZKey::from(&correct);

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_fullmove_counter() {
        let board = BoardBuilder::default().fullmove_counter(5).build();
        let correct = Board {
            fullmove_counter: 5,
            ..BoardBuilder::construct_starting_board().build()
        };

        assert_eq!(board, correct);
    }
}
