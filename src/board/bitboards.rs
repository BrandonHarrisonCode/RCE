use super::piece::{Color, Kind};
use super::square::Square;

pub mod builder;

use builder::Builder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitBoards {
    pub white_pawns: u64,
    pub white_king: u64,
    pub white_queens: u64,
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub black_pawns: u64,
    pub black_king: u64,
    pub black_queens: u64,
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,

    pub white_pieces: u64,
    pub black_pieces: u64,
    pub all_pieces: u64,
}

impl Default for BitBoards {
    fn default() -> Self {
        Self::new()
    }
}

impl BitBoards {
    pub const fn new() -> Self {
        Self {
            white_pawns: 0,
            white_king: 0,
            white_queens: 0,
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            black_pawns: 0,
            black_king: 0,
            black_queens: 0,
            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,

            white_pieces: 0,
            black_pieces: 0,
            all_pieces: 0,
        }
    }

    pub const fn default() -> Self {
        let white_pawns =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        let white_king = 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        let white_queens =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        let white_rooks =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
        let white_bishops =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
        let white_knights =
            0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
        let black_pawns =
            0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_king = 0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_queens =
            0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
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
            white_pawns,
            white_king,
            white_queens,
            white_rooks,
            white_knights,
            white_bishops,
            black_pawns,
            black_king,
            black_queens,
            black_rooks,
            black_knights,
            black_bishops,

            white_pieces,
            black_pieces,
            all_pieces,
        }
    }

    pub const fn builder() -> Builder {
        Builder::default()
    }

    fn recompute_combinations(&mut self, kind: Option<Kind>) {
        if kind.is_none() || kind.is_some_and(|k| k.get_color() == Color::White) {
            self.white_pieces = self.white_pawns
                | self.white_knights
                | self.white_bishops
                | self.white_rooks
                | self.white_queens
                | self.white_king;
        }
        if kind.is_none() || kind.is_some_and(|k| k.get_color() == Color::Black) {
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
        let mask = square.get_mask();

        if mask & self.white_pieces != 0 {
            if mask & self.white_pawns != 0 {
                return Some(Kind::Pawn(Color::White));
            } else if mask & self.white_king != 0 {
                return Some(Kind::King(Color::White));
            } else if mask & self.white_queens != 0 {
                return Some(Kind::Queen(Color::White));
            } else if mask & self.white_rooks != 0 {
                return Some(Kind::Rook(Color::White));
            } else if mask & self.white_knights != 0 {
                return Some(Kind::Knight(Color::White));
            } else if mask & self.white_bishops != 0 {
                return Some(Kind::Bishop(Color::White));
            }
            unreachable!("White pieces collection is malformed! Detected a white piece at square {square}, but no piece was found!")
        } else if mask & self.black_pieces != 0 {
            if mask & self.black_pawns != 0 {
                return Some(Kind::Pawn(Color::Black));
            } else if mask & self.black_king != 0 {
                return Some(Kind::King(Color::Black));
            } else if mask & self.black_queens != 0 {
                return Some(Kind::Queen(Color::Black));
            } else if mask & self.black_rooks != 0 {
                return Some(Kind::Rook(Color::Black));
            } else if mask & self.black_knights != 0 {
                return Some(Kind::Knight(Color::Black));
            } else if mask & self.black_bishops != 0 {
                return Some(Kind::Bishop(Color::Black));
            }
            unreachable!("black pieces collection is malformed! Detected a black piece at square {square}, but no piece was found!")
        } else {
            None
        }
    }

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

        self.recompute_combinations(Some(kind));
    }

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

        self.recompute_combinations(Some(kind));
    }
}
