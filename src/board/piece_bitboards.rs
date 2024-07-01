use super::bitboard::Bitboard;
use super::piece::{Color, Kind};
use super::square::Square;

pub mod builder;

use builder::Builder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceBitboards {
    pub white_pawns: Bitboard,
    pub white_king: Bitboard,
    pub white_queens: Bitboard,
    pub white_rooks: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub black_pawns: Bitboard,
    pub black_king: Bitboard,
    pub black_queens: Bitboard,
    pub black_rooks: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,

    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub all_pieces: Bitboard,
}

impl Default for PieceBitboards {
    fn default() -> Self {
        Self::new()
    }
}

impl PieceBitboards {
    pub const fn new() -> Self {
        Self {
            white_pawns: Bitboard::new(0),
            white_king: Bitboard::new(0),
            white_queens: Bitboard::new(0),
            white_rooks: Bitboard::new(0),
            white_knights: Bitboard::new(0),
            white_bishops: Bitboard::new(0),
            black_pawns: Bitboard::new(0),
            black_king: Bitboard::new(0),
            black_queens: Bitboard::new(0),
            black_rooks: Bitboard::new(0),
            black_knights: Bitboard::new(0),
            black_bishops: Bitboard::new(0),

            white_pieces: Bitboard::new(0),
            black_pieces: Bitboard::new(0),
            all_pieces: Bitboard::new(0),
        }
    }

    /// Instantiates a new `BitBoards` refering to the game start state
    ///
    /// # Examples
    /// ```
    /// let board = Board::default();
    /// ```
    pub const fn default() -> Self {
        let white_pawns =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        let white_king = 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        let white_queens =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        let white_rooks =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
        let white_bishops =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
        let white_knights =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
        let black_pawns =
            0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_king = 0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_queens =
            0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_rooks =
            0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_bishops =
            0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_knights =
            0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        let white_pieces =
            white_pawns | white_king | white_queens | white_rooks | white_bishops | white_knights;
        let black_pieces =
            black_pawns | black_king | black_queens | black_rooks | black_bishops | black_knights;
        let all_pieces = white_pieces | black_pieces;

        Self {
            white_pawns: Bitboard::new(white_pawns),
            white_king: Bitboard::new(white_king),
            white_queens: Bitboard::new(white_queens),
            white_rooks: Bitboard::new(white_rooks),
            white_knights: Bitboard::new(white_knights),
            white_bishops: Bitboard::new(white_bishops),
            black_pawns: Bitboard::new(black_pawns),
            black_king: Bitboard::new(black_king),
            black_queens: Bitboard::new(black_queens),
            black_rooks: Bitboard::new(black_rooks),
            black_knights: Bitboard::new(black_knights),
            black_bishops: Bitboard::new(black_bishops),

            white_pieces: Bitboard::new(white_pieces),
            black_pieces: Bitboard::new(black_pieces),
            all_pieces: Bitboard::new(all_pieces),
        }
    }

    /// Creates a `Builder` object for constructing new `BitBoards`
    ///
    /// # Examples
    /// ```
    /// let builder = BitBoards::builder();
    /// let bitboards = builder.build();
    /// ```
    #[allow(dead_code)]
    pub const fn builder() -> Builder {
        Builder::default()
    }

    /// Recomputes the meta data bitboards of `white_pieces`, `black_pieces`, and `all_pieces`
    ///
    /// This is useful after changing one of the more granular bitboards. This
    /// method must be called after each change in order to keep the meta
    /// bitboards up to date.
    ///
    /// # Arguments
    ///
    /// * `color` - An optional `Color` that is used to cut down on recomputing unnescessary metadata.
    ///
    /// # Examples
    /// ```
    /// let bb = BitBoards::default();
    /// bb.white_pawns = 1;
    /// bb.recompute_combinatations();
    /// ```
    fn recompute_combinations(&mut self, color: Option<Color>) {
        if color.is_none() || color.is_some_and(|c| c == Color::White) {
            self.white_pieces = self.white_pawns
                | self.white_knights
                | self.white_bishops
                | self.white_rooks
                | self.white_queens
                | self.white_king;
        }
        if color.is_none() || color.is_some_and(|c| c == Color::Black) {
            self.black_pieces = self.black_pawns
                | self.black_knights
                | self.black_bishops
                | self.black_rooks
                | self.black_queens
                | self.black_king;
        }

        self.all_pieces = self.white_pieces | self.black_pieces;
    }

    /// Returns a `PieceKind` Option of the piece currently occupying `square`
    ///
    /// # Arguments
    ///
    /// * `square` - A square on the board we would like to inspect
    ///
    /// # Errors
    /// Returns `None` if there is no piece at the specified square.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert_eq!(PieceKind::Rook(Color::White), board.get_piece(Square::new("a1")));
    /// assert_eq!(None, board.get_piece(Square::new("b3")));
    /// ``singleton`
    pub fn get_piece_kind(&self, square: Square) -> Option<Kind> {
        let mask = Bitboard::new(square.get_mask());

        if !(mask & self.white_pieces).is_empty() {
            if !(mask & self.white_pawns).is_empty() {
                return Some(Kind::Pawn(Color::White));
            } else if !(mask & self.white_king).is_empty() {
                return Some(Kind::King(Color::White));
            } else if !(mask & self.white_queens).is_empty() {
                return Some(Kind::Queen(Color::White));
            } else if !(mask & self.white_rooks).is_empty() {
                return Some(Kind::Rook(Color::White));
            } else if !(mask & self.white_knights).is_empty() {
                return Some(Kind::Knight(Color::White));
            } else if !(mask & self.white_bishops).is_empty() {
                return Some(Kind::Bishop(Color::White));
            }
            unreachable!("White pieces collection is malformed! Detected a white piece at square {square}, but no piece was found!")
        } else if !(mask & self.black_pieces).is_empty() {
            if !(mask & self.black_pawns).is_empty() {
                return Some(Kind::Pawn(Color::Black));
            } else if !(mask & self.black_king).is_empty() {
                return Some(Kind::King(Color::Black));
            } else if !(mask & self.black_queens).is_empty() {
                return Some(Kind::Queen(Color::Black));
            } else if !(mask & self.black_rooks).is_empty() {
                return Some(Kind::Rook(Color::Black));
            } else if !(mask & self.black_knights).is_empty() {
                return Some(Kind::Knight(Color::Black));
            } else if !(mask & self.black_bishops).is_empty() {
                return Some(Kind::Bishop(Color::Black));
            }
            unreachable!("Black pieces collection is malformed! Detected a black piece at square {square}, but no piece was found!")
        } else {
            None
        }
    }

    /// Adds the specified piece kind to the specified square
    ///
    /// # Arguments
    ///
    /// * `square` - The designated square to add the piece to.
    ///
    /// * `kind` - The piece kind to add to the square.
    ///
    /// # Examples
    /// ```
    /// let bb = BitBoards::default();
    /// bb.add_piece(Square("a4"), Kind::Rook(Color::White))
    /// ```
    pub fn add_piece(&mut self, square: Square, kind: Kind) {
        let mask = square.get_mask();

        match kind {
            Kind::Pawn(Color::White) => self.white_pawns |= mask,
            Kind::Knight(Color::White) => self.white_knights |= mask,
            Kind::Bishop(Color::White) => self.white_bishops |= mask,
            Kind::Rook(Color::White) => self.white_rooks |= mask,
            Kind::Queen(Color::White) => self.white_queens |= mask,
            Kind::King(Color::White) => self.white_king |= mask,
            Kind::Pawn(Color::Black) => self.black_pawns |= mask,
            Kind::Knight(Color::Black) => self.black_knights |= mask,
            Kind::Bishop(Color::Black) => self.black_bishops |= mask,
            Kind::Rook(Color::Black) => self.black_rooks |= mask,
            Kind::Queen(Color::Black) => self.black_queens |= mask,
            Kind::King(Color::Black) => self.black_king |= mask,
        }

        self.recompute_combinations(Some(kind.get_color()));
    }

    /// Removes the specified piece kind from the square.
    ///
    /// # Arguments
    ///
    /// * `square` - The square to remove the piece from.
    ///
    /// * `kind` - The piece kind to remove from the square.
    ///
    /// # Examples
    /// ```
    /// let bb = BitBoards::default();
    /// bb.remove_piece(Square("a2"), Kind::Pawn(Color::White));
    /// ```
    pub fn remove_piece(&mut self, square: Square, kind: Kind) {
        let mask = !square.get_mask();

        match kind {
            Kind::Pawn(Color::White) => self.white_pawns &= mask,
            Kind::Knight(Color::White) => self.white_knights &= mask,
            Kind::Bishop(Color::White) => self.white_bishops &= mask,
            Kind::Rook(Color::White) => self.white_rooks &= mask,
            Kind::Queen(Color::White) => self.white_queens &= mask,
            Kind::King(Color::White) => self.white_king &= mask,
            Kind::Pawn(Color::Black) => self.black_pawns &= mask,
            Kind::Knight(Color::Black) => self.black_knights &= mask,
            Kind::Bishop(Color::Black) => self.black_bishops &= mask,
            Kind::Rook(Color::Black) => self.black_rooks &= mask,
            Kind::Queen(Color::Black) => self.black_queens &= mask,
            Kind::King(Color::Black) => self.black_king &= mask,
        }

        self.recompute_combinations(Some(kind.get_color()));
    }
}
