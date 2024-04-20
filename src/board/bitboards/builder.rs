use super::BitBoards;
use crate::board::piece::{Color, Kind};

#[derive(Default)]
pub struct BitBoardsBuilder {
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

    pub white_pieces: u64,
    pub black_pieces: u64,
    pub all_pieces: u64,
}

impl BitBoardsBuilder {
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

            white_pieces: 0,
            black_pieces: 0,
            all_pieces: 0,
        }
    }

    /// Consume the `BitBoardsBuilder` to create a `BitBoard`
    ///
    /// # Returns
    ///
    /// * `BitBoards` - The BitBoards represented by the builder
    ///
    /// # Example
    ///
    /// ```
    /// use crate::board::{Color, Castling};
    /// use crate::bitboards::BitBoards;
    ///
    /// let bitboards: BitBoards = BitBoardsBuilder::default().pawns(Color::White, 5).build();
    /// ```
    pub fn build(&mut self) -> BitBoards {
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
        BitBoards {
            white_pawns: self.white_pawns,
            white_king: self.white_king,
            white_queens: self.white_queens,
            white_rooks: self.white_rooks,
            white_bishops: self.white_bishops,
            white_knights: self.white_knights,
            black_pawns: self.black_pawns,
            black_king: self.black_king,
            black_queens: self.black_queens,
            black_rooks: self.black_rooks,
            black_bishops: self.black_bishops,
            black_knights: self.black_knights,

            white_pieces,
            black_pieces,
            all_pieces,
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
