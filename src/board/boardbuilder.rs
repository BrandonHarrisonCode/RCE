use super::piece::Color;
use super::ply::Ply;
use super::Board;
use super::CastlingStatus;
use super::GameState;

use super::bitboards::builder::Builder;
use super::bitboards::Bitboards;

#[derive(Default)]
pub struct BoardBuilder {
    pub current_turn: Color,
    pub halfmove_clock: u8,
    pub fullmove_counter: u16,
    pub game_state: GameState,

    pub white_kingside_castling: CastlingStatus,
    pub white_queenside_castling: CastlingStatus,
    pub black_kingside_castling: CastlingStatus,
    pub black_queenside_castling: CastlingStatus,

    pub en_passant_file: Option<u8>,

    pub bitboards: Builder,

    pub history: Vec<Ply>,
}

impl BoardBuilder {
    #[allow(dead_code)]
    pub const fn default() -> Self {
        Self {
            current_turn: Color::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,
            game_state: GameState::InProgress,

            white_kingside_castling: CastlingStatus::Availiable,
            white_queenside_castling: CastlingStatus::Availiable,
            black_kingside_castling: CastlingStatus::Availiable,
            black_queenside_castling: CastlingStatus::Availiable,

            en_passant_file: None,

            bitboards: Bitboards::builder(),

            history: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub const fn game_state(mut self, state: GameState) -> Self {
        self.game_state = state;
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

    /// Set the kingside castling rights of the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set the kingside castling rights for
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
    /// let builder = BoardBuilder::default().kingside_castling(Color::Black, Castling::Unavailiable);
    /// ```
    pub const fn kingside_castling(mut self, color: Color, value: CastlingStatus) -> Self {
        match color {
            Color::White => self.white_kingside_castling = value,
            Color::Black => self.black_kingside_castling = value,
        }
        self
    }

    /// Set the queenside castling rights of the specified color
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set the queenside castling rights for
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
    /// let builder = BoardBuilder::default().queenside_castling(Color::Black, Castling::Unavailiable);
    /// ```
    pub const fn queenside_castling(mut self, color: Color, value: CastlingStatus) -> Self {
        match color {
            Color::White => self.white_queenside_castling = value,
            Color::Black => self.black_queenside_castling = value,
        }
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
        self.history = history.to_vec();
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
    pub const fn halfmove_clock(mut self, value: u8) -> Self {
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
    pub fn build(&self) -> Board {
        Board {
            current_turn: self.current_turn,
            halfmove_clock: self.halfmove_clock,
            fullmove_counter: self.fullmove_counter,
            game_state: self.game_state,

            white_kingside_castling: self.white_kingside_castling,
            white_queenside_castling: self.white_queenside_castling,
            black_kingside_castling: self.black_kingside_castling,
            black_queenside_castling: self.black_queenside_castling,

            en_passant_file: self.en_passant_file,

            history: self.history.clone(),
            bitboards: self.bitboards.build(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::bitboard::Bitboard;
    use super::super::square::Square;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn board_builder_default() {
        let board = BoardBuilder::default().build();
        let correct = Board::construct_empty_board();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_turn() {
        let board = BoardBuilder::default().turn(Color::Black).build();
        let correct = Board {
            current_turn: Color::Black,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_turn() {
        let board = BoardBuilder::default()
            .turn(Color::Black)
            .turn(Color::White)
            .build();
        let correct = Board::construct_empty_board();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_kingside_castling() {
        let board = BoardBuilder::default()
            .kingside_castling(Color::White, CastlingStatus::Unavailiable)
            .build();
        let correct = Board {
            white_kingside_castling: CastlingStatus::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_kingside_castling() {
        let board = BoardBuilder::default()
            .kingside_castling(Color::Black, CastlingStatus::Unavailiable)
            .build();
        let correct = Board {
            black_kingside_castling: CastlingStatus::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_queenside_castling() {
        let board = BoardBuilder::default()
            .queenside_castling(Color::White, CastlingStatus::Unavailiable)
            .build();
        let correct = Board {
            white_queenside_castling: CastlingStatus::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_queenside_castling() {
        let board = BoardBuilder::default()
            .queenside_castling(Color::Black, CastlingStatus::Unavailiable)
            .build();
        let correct = Board {
            black_queenside_castling: CastlingStatus::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_pawns() {
        let board = BoardBuilder::default()
            .pawns(Color::White, 1)
            .pawns(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_pawns: Bitboard::new(1),
                black_pawns: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_king() {
        let board = BoardBuilder::default()
            .king(Color::White, 1)
            .king(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_king: Bitboard::new(1),
                black_king: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };
        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_queens() {
        let board = BoardBuilder::default()
            .queens(Color::White, 1)
            .queens(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_queens: Bitboard::new(1),
                black_queens: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_rooks() {
        let board = BoardBuilder::default()
            .rooks(Color::White, 1)
            .rooks(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_rooks: Bitboard::new(1),
                black_rooks: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_bishops() {
        let board = BoardBuilder::default()
            .bishops(Color::White, 1)
            .bishops(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_bishops: Bitboard::new(1),
                black_bishops: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_knights() {
        let board = BoardBuilder::default()
            .knights(Color::White, 1)
            .knights(Color::Black, 2)
            .build();
        let correct = Board {
            bitboards: Bitboards {
                white_knights: Bitboard::new(1),
                black_knights: Bitboard::new(2),
                white_pieces: Bitboard::new(1),
                black_pieces: Bitboard::new(2),
                all_pieces: Bitboard::new(1 | 2),
                ..Default::default()
            },
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_history() {
        let history = vec![Ply::new(Square::from("a1"), Square::from("a2"))];
        let board = BoardBuilder::default().history(&history).build();
        let correct = Board {
            history,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_en_passant() {
        let board = BoardBuilder::default().en_passant_file(Some(1)).build();
        let correct = Board {
            en_passant_file: Some(1),
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_halfmove_clock() {
        let board = BoardBuilder::default().halfmove_clock(5).build();
        let correct = Board {
            halfmove_clock: 5,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_fullmove_counter() {
        let board = BoardBuilder::default().fullmove_counter(5).build();
        let correct = Board {
            fullmove_counter: 5,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }
}
