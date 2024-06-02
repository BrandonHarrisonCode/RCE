use super::super::bitboard::Bitboard;
use super::Bitboards;
use crate::board::piece::Color;

#[derive(Default)]
pub struct Builder {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Builder {
    #[allow(dead_code)]
    pub const fn default() -> Self {
        Self {
            white_pawns: 0,
            white_king: 0,
            white_queens: 0,
            white_rooks: 0,
            white_bishops: 0,
            white_knights: 0,
            black_pawns: 0,
            black_king: 0,
            black_queens: 0,
            black_rooks: 0,
            black_bishops: 0,
            black_knights: 0,
        }
    }

    /// Consume the `Builder` to create a `BitBoard`
    ///
    /// # Returns
    ///
    /// * `BitBoards` - The `BitBoards` represented by the builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{Color, Castling};
    /// use crate::bitboards::BitBoards;
    ///
    /// let bitboards: BitBoards = BitBoardsBuilder::default().pawns(Color::White, 5).build();
    /// ```
    pub fn build(&mut self) -> Bitboards {
        let white_pieces = self.white_pawns
            | self.white_king
            | self.white_queens
            | self.white_rooks
            | self.white_bishops
            | self.white_knights;
        let black_pieces = self.black_pawns
            | self.black_king
            | self.black_queens
            | self.black_rooks
            | self.black_bishops
            | self.black_knights;
        let all_pieces = white_pieces | black_pieces;
        Bitboards {
            white_pawns: Bitboard::new(self.white_pawns),
            white_king: Bitboard::new(self.white_king),
            white_queens: Bitboard::new(self.white_queens),
            white_rooks: Bitboard::new(self.white_rooks),
            white_bishops: Bitboard::new(self.white_bishops),
            white_knights: Bitboard::new(self.white_knights),
            black_pawns: Bitboard::new(self.black_pawns),
            black_king: Bitboard::new(self.black_king),
            black_queens: Bitboard::new(self.black_queens),
            black_rooks: Bitboard::new(self.black_rooks),
            black_bishops: Bitboard::new(self.black_bishops),
            black_knights: Bitboard::new(self.black_knights),

            white_pieces: Bitboard::new(white_pieces),
            black_pieces: Bitboard::new(black_pieces),
            all_pieces: Bitboard::new(all_pieces),
        }
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
        match color {
            Color::White => self.white_pawns = value,
            Color::Black => self.black_pawns = value,
        }
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
        match color {
            Color::White => self.white_king = value,
            Color::Black => self.black_king = value,
        }
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
        match color {
            Color::White => self.white_queens = value,
            Color::Black => self.black_queens = value,
        }
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
        match color {
            Color::White => self.white_rooks = value,
            Color::Black => self.black_rooks = value,
        }
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
        match color {
            Color::White => self.white_bishops = value,
            Color::Black => self.black_bishops = value,
        }
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
        match color {
            Color::White => self.white_knights = value,
            Color::Black => self.black_knights = value,
        }
        self
    }
}
