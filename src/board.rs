use std::fmt;
pub mod bitboard;
pub mod boardbuilder;
pub mod piece;
mod piece_bitboards;
pub mod ply;
pub mod serialize;
pub mod square;
pub mod transposition_table;
pub mod zkey;

use bitboard::Bitboard;
#[allow(clippy::module_name_repetitions)]
pub use boardbuilder::BoardBuilder;
use piece::{Color, Kind};
use piece_bitboards::PieceBitboards;
use ply::castling::{CastlingKind, CastlingStatus};
pub use ply::Ply;
use square::Square;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Unknown,
    InProgress,
    CheckmateWhite,
    CheckmateBlack,
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
}

/// A board object, representing all of the state of the game
/// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub current_turn: Color,
    pub fullmove_counter: u16,
    pub game_state: GameState,

    en_passant_file: Option<u8>,

    pub bitboards: PieceBitboards,

    history: Vec<Ply>,
}

impl Default for Board {
    /// Creates a new board object that represents the starting board state in a normal game
    ///
    /// # Examples
    /// ```
    /// let board = Board::default();
    /// ```
    fn default() -> Self {
        Self {
            current_turn: Color::White,
            fullmove_counter: 1,
            game_state: GameState::InProgress,

            bitboards: PieceBitboards::default(),

            en_passant_file: None,

            history: vec![Ply::default()],
        }
    }
}

impl Board {
    /// Creates a `BoardBuilder` object to construct a new board
    ///
    /// # Examples
    /// ```
    /// let board = Board::builder().kingside_castling(true, true).build();
    /// ```
    pub fn builder() -> BoardBuilder {
        BoardBuilder::default()
    }

    /// Returns a boolean representing whether or not the current player has castling rights
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert_eq!(board.castle_status(CastlingKind::WhiteKingside), Castling::Availiable);
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn castle_status(&self, kind: CastlingKind) -> CastlingStatus {
        match kind {
            CastlingKind::WhiteKingside => {
                self.history.last().unwrap().castling_rights.white_kingside
            }
            CastlingKind::WhiteQueenside => {
                self.history.last().unwrap().castling_rights.white_queenside
            }
            CastlingKind::BlackKingside => {
                self.history.last().unwrap().castling_rights.black_kingside
            }
            CastlingKind::BlackQueenside => {
                self.history.last().unwrap().castling_rights.black_queenside
            }
        }
    }

    /// Returns a list of all potential moves for the current side
    ///
    /// The list is not guaranteed to be legal, and may include moves that would
    /// leave the king in check, or moves that illegally capture the player's
    /// own pieces, move through their own pieces, etc. This function is not
    /// usually called on its own. It is normally paired with `filter_moves()`
    /// to create a legal moveset of the board.
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let movelist = board.get_all_moves(Square::new("a2"));
    /// ```
    fn get_all_moves(&self) -> Vec<Ply> {
        let mut all_moves = Vec::new();

        for square_idx in 0..64u8 {
            let square = Square::from(square_idx);
            if let Some(piece) = self.get_piece(square) {
                if self.current_turn != piece.get_color() {
                    continue;
                }

                all_moves.append(
                    &mut piece
                        .get_moveset(square, self)
                        .into_iter()
                        .map(|mut mv| {
                            if mv.en_passant {
                                mv.captured_piece = self.get_piece(Square {
                                    rank: mv.start.rank,
                                    file: mv.dest.file,
                                });
                            } else {
                                mv.captured_piece = self.get_piece(mv.dest);
                            }

                            mv
                        })
                        .collect::<Vec<Ply>>(),
                );
            }
        }

        all_moves
    }

    /// Returns a list of all legal moves for the current side
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let movelist = board.get_all_moves(Square::new("a2"));
    /// ```
    pub fn get_legal_moves(&mut self) -> Vec<Ply> {
        self.get_all_moves()
            .into_iter()
            .filter(|mv| self.is_legal_move(*mv).is_ok())
            .collect()
    }

    /// Returns a boolean representing whether or not a given move is legal
    ///
    /// The move is only considered legal if it does not leave the king in check
    ///
    /// # Examples
    /// ```
    /// let ply = Ply(Square::new("e2"), Square::new("e9"));
    /// assert!(!board.is_legal_move(ply));
    /// ```
    fn is_legal_move(&mut self, ply: Ply) -> Result<Ply, &'static str> {
        self.make_move(ply);
        if self.is_in_check(self.current_turn.opposite()) {
            self.unmake_move();
            return Err("Move is not valid. The move would leave the king in check.");
        }
        self.unmake_move();

        Ok(ply)
    }

    /// Switches the current turn to the other player
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// board.switch_turn();
    /// assert_eq!(Color::Black, board.current_turn);
    /// board.switch_turn();
    /// assert_eq!(Color::White, board.current_turn);
    /// ```
    pub const fn switch_turn(&mut self) {
        self.current_turn = self.current_turn.opposite();
    }

    /// Returns a `CastlingStatus` representing whether or not the current `kind` of castling is availiable
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of castling to check for
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert_eq!(CastlingStatus::Availiable, board.castling_ability(CastlingKind::WhiteKingside));
    /// assert_eq!(CastlingStatus::Availiable, board.castling_ability(CastlingKind::BlackQueenside));
    /// ```
    pub fn castling_ability(&self, kind: CastlingKind) -> Result<CastlingStatus, &'static str> {
        match (kind, self.current_turn) {
            (CastlingKind::WhiteKingside | CastlingKind::WhiteQueenside, Color::Black)
            | (CastlingKind::BlackKingside | CastlingKind::BlackQueenside, Color::White) => {
                return Err("Cannot castle when it is not your turn.");
            }
            _ => (),
        }

        if self.castle_status(kind) == CastlingStatus::Available
            && self
                .no_pieces_between_castling(kind)
                .and(self.no_checks_castling(kind))
                .is_ok()
        {
            Ok(CastlingStatus::Available)
        } else {
            Ok(CastlingStatus::Unavailable)
        }
    }

    /// Returns a Result representing whether or not there are no squares with pieces on the castling path
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of castling to check for
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert!(board.no_pieces_between_castling(CastlingKind::WhiteKingside).is_err());
    /// assert!(board.no_pieces_between_castling(CastlingKind::BlackQueenside).is_err());
    /// ```
    fn no_pieces_between_castling(&self, kind: CastlingKind) -> Result<(), &'static str> {
        let pieces_blocking = match kind {
            CastlingKind::WhiteKingside => self.bitboards.all_pieces & 0x60,
            CastlingKind::WhiteQueenside => self.bitboards.all_pieces & 0xE,
            CastlingKind::BlackKingside => self.bitboards.all_pieces & 0x_60000000_00000000,
            CastlingKind::BlackQueenside => self.bitboards.all_pieces & 0x_0E00_0000_0000_0000,
        };

        if pieces_blocking.is_empty() {
            Ok(())
        } else {
            Err("There are pieces between the start and destination squares.")
        }
    }

    /// Returns a Result representing whether or not there are no squares in check between two squares
    ///
    /// This method is mostly used for calculating castling rights
    ///
    /// # Arguments
    ///
    /// * `start` - The starting square
    /// * `dest` - The destination square
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert!(board.no_checks_between(Square::new("a1"), Square::new("h1")).is_ok());
    /// assert!(board.no_checks_between(Square::new("a8"), Square::new("h8")).is_err());
    /// ```
    fn no_checks_castling(&self, kind: CastlingKind) -> Result<(), &'static str> {
        let attacks = self.get_attacked_squares(self.current_turn);
        if match kind {
            CastlingKind::WhiteKingside => (attacks & 0x70).is_empty(),
            CastlingKind::WhiteQueenside => (attacks & 0x1C).is_empty(),
            CastlingKind::BlackKingside => (attacks & 0x_7000_0000_0000_0000).is_empty(),
            CastlingKind::BlackQueenside => (attacks & 0x1C00_0000_0000_0000).is_empty(),
        } {
            Ok(())
        } else {
            Err("There are checks between the start and destination squares.")
        }
    }

    /// Returns a bitboard representing all squares that are attacked from `color`'s perspective
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the player to calculate the attacked squares for
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    ///
    /// let attacked_squares = board.get_attacked_squares(Color::White);
    /// ```
    #[allow(clippy::literal_string_with_formatting_args)]
    fn get_attacked_squares(&self, color: Color) -> Bitboard {
        let attacking_pieces = match color {
            Color::White => self.bitboards.black_pieces,
            Color::Black => self.bitboards.white_pieces,
        };

        let mut attacks = Bitboard::new(0);
        for square in 0..64u8 {
            if attacking_pieces & (1 << square) == Bitboard::new(0) {
                continue;
            }

            let piece = self
                .get_piece(Square::from(square))
                .expect("No piece found at {square} where bitboard claimed piece was!");

            attacks |= piece.get_attacks(Square::from(square), self);
        }

        attacks
    }

    /// Returns the halfmove clock of the current board state
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert_eq!(0, board.get_halfmove_clock());
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn get_halfmove_clock(&self) -> u16 {
        self.history
            .last()
            .expect("Board should always have one history")
            .halfmove_clock
    }

    /// Returns a boolean representing whether or not the current side is in check
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert!(!board.is_in_check());
    /// ```
    pub fn is_in_check(&self, color: Color) -> bool {
        let attacks = self.get_attacked_squares(color);

        let king_pos = match color {
            Color::White => self.bitboards.white_king,
            Color::Black => self.bitboards.black_king,
        };

        !(king_pos & attacks).is_empty()
    }

    #[allow(dead_code)]
    /// Returns a boolean representing whether or not the current game is over
    pub fn is_game_over(&mut self) -> bool {
        self.set_game_state();
        self.game_state != GameState::InProgress
    }

    /// Sets the game state by evaluating the board for checkmate, stalemate, threefold repetition, and the fifty move rule
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// board.set_game_state();
    /// assert_eq!(GameState::InProgress, board.game_state);
    /// ```
    fn set_game_state(&mut self) {
        if self.game_state != GameState::Unknown {
            return;
        }

        let is_in_check = self.is_in_check(self.current_turn);
        let legal_moves_empty = self.get_legal_moves().is_empty();
        //let threefold_repetition = self.is_threefold_repetition();
        let threefold_repetition = false;

        match (
            is_in_check,
            legal_moves_empty,
            self.get_halfmove_clock() >= 100,
            threefold_repetition,
        ) {
            (true, true, _, _) => {
                self.game_state = match self.current_turn {
                    Color::White => GameState::CheckmateWhite,
                    Color::Black => GameState::CheckmateBlack,
                };
            }
            (false, true, _, _) => self.game_state = GameState::Stalemate,
            (_, _, true, _) => self.game_state = GameState::FiftyMoveRule,
            (_, _, _, true) => self.game_state = GameState::ThreefoldRepetition,
            (_, false, false, false) => {
                self.game_state = GameState::InProgress;
            }
        }
    }

    #[allow(dead_code)]
    /// Returns the winner of the game, if there is one
    pub fn get_winner(&mut self) -> Option<Color> {
        self.set_game_state();
        match self.game_state {
            GameState::CheckmateWhite => Some(Color::Black),
            GameState::CheckmateBlack => Some(Color::White),
            _ => None,
        }
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
    /// let board = BoardBuilder::construct_starting_board().build();
    /// assert_eq!(PieceKind::Rook(Color::White), board.get_piece(Square::new("a1")));
    /// assert_eq!(None, board.get_piece(Square::new("b3")));
    /// ```
    pub fn get_piece(&self, square: Square) -> Option<Kind> {
        self.bitboards.get_piece_kind(square)
    }

    /// Adds a new piece of the specified kind to a square on the board
    ///
    /// # Arguments
    ///
    /// * `square` - A square on the board to place the piece
    ///
    /// * `piece` - The type of piece to place at this square
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// board.add_piece(&Square::new("a3"), &PieceKind::Rook(Color::White));
    /// ```
    pub fn add_piece(&mut self, square: Square, piece: Kind) {
        self.bitboards.add_piece(square, piece);
    }

    /// Remove a specific kind of piece from the board at the specified square
    ///
    /// # Arguments
    ///
    /// * `square` - A square on the board to clear
    ///
    /// * `piece` - The type of piece to remove from the square
    ///
    /// # Panics
    /// Will panic if there is no piece at the expected square.
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// // Playing with rook odds
    /// board.remove_piece(&Square::new("a1"), &PieceKind::Rook(Color::White));
    /// ```
    pub fn remove_piece(&mut self, square: Square, piece: Kind) {
        self.bitboards.remove_piece(square, piece);
    }

    /// Replaces the piece at the dest square with the piece at the destination square
    ///
    /// # ArgumentSome(file)s
    ///
    /// * `origin` - The square of the piece to move
    /// * `to_replace` - The square to move the piece to
    ///
    /// # Returns
    ///
    /// An Option of the piece kind that was replaced, if any
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let captured_piece = board.replace_square(Square::new("e2"), Square::new("e4"));
    /// ```
    fn replace_square(&mut self, origin: Square, to_replace: Square) -> Option<Kind> {
        let start_piece_kind = self.get_piece(origin).unwrap();
        self.remove_piece(origin, start_piece_kind);

        let dest_piece_kind_option = self.get_piece(to_replace);
        if let Some(dest_piece_kind) = dest_piece_kind_option {
            self.remove_piece(to_replace, dest_piece_kind);
        }

        self.add_piece(to_replace, start_piece_kind);

        dest_piece_kind_option
    }

    /// Finds the move in the list of all legal moves that corresponds to the given notation
    pub fn find_move(&mut self, notation: &str) -> Result<Ply, &'static str> {
        self.get_legal_moves()
            .into_iter()
            .find(|m| m.to_notation() == notation)
            .ok_or("Move not found")
    }

    /// Makes a half-move on this board
    ///
    /// # Arguments
    ///
    /// * `new_move` - A Ply that holds the origin and destination square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// // Move the a pawn one square forward
    /// board.make_move(Ply::new(Square::new("a2"), Square::new("a3")));
    /// ```
    #[allow(clippy::too_many_lines)]
    pub fn make_move(&mut self, mut new_move: Ply) {
        let previous_move: Ply = self.history.last().copied().unwrap_or_default();
        new_move.halfmove_clock = previous_move.halfmove_clock + 1;
        new_move.castling_rights = previous_move.castling_rights;

        self.make_move_en_passant_checks(&new_move);

        if let (Some(promoted_to), Some(Kind::Pawn(c))) =
            (new_move.promoted_to, self.get_piece(new_move.dest))
        {
            self.remove_piece(new_move.dest, Kind::Pawn(c));
            self.add_piece(new_move.dest, promoted_to);
        }

        self.make_move_castling_checks(&mut new_move);

        self.game_state = GameState::Unknown;
        self.switch_turn();
        if self.current_turn == Color::White {
            self.fullmove_counter += 1;
        }
        self.history.push(new_move);
    }

    /// Handles En Passant related logic for making moves
    fn make_move_en_passant_checks(&mut self, new_move: &Ply) {
        if new_move.is_double_pawn_push {
            self.en_passant_file = Some(new_move.dest.file);
        } else {
            self.en_passant_file = None;
        }

        let dest_piece_kind = self.replace_square(new_move.start, new_move.dest);
        if new_move.en_passant {
            self.remove_piece(
                Square {
                    file: new_move.dest.file,
                    rank: new_move.start.rank,
                },
                Kind::Pawn(self.current_turn.opposite()),
            );
        } else {
            assert_eq!(new_move.captured_piece, dest_piece_kind);
        }
    }

    /// Handles Castling related logic for making moves
    fn make_move_castling_checks(&mut self, new_move: &mut Ply) {
        if new_move.is_castles {
            let (rook_start, rook_dest) = match new_move.dest {
                Square { rank: 0, file: 6 } => (Square::from("h1"), Square::from("f1")),
                Square { rank: 0, file: 2 } => (Square::from("a1"), Square::from("d1")),
                Square { rank: 7, file: 6 } => (Square::from("h8"), Square::from("f8")),
                Square { rank: 7, file: 2 } => (Square::from("a8"), Square::from("d8")),
                _ => panic!("Invalid castling king destination {}", new_move.dest),
            };

            self.replace_square(rook_start, rook_dest);

            match new_move.dest {
                Square {
                    rank: 0,
                    file: 6 | 2,
                } => {
                    new_move.castling_rights.white_kingside = CastlingStatus::Unavailable;
                    new_move.castling_rights.white_queenside = CastlingStatus::Unavailable;
                }
                Square {
                    rank: 7,
                    file: 6 | 2,
                } => {
                    new_move.castling_rights.black_kingside = CastlingStatus::Unavailable;
                    new_move.castling_rights.black_queenside = CastlingStatus::Unavailable;
                }
                _ => panic!("Invalid castling king destination {}", new_move.dest),
            }
        } else if matches!(self.get_piece(new_move.dest), Some(Kind::King(_))) {
            match self.current_turn {
                Color::White => {
                    new_move.castling_rights.white_kingside = CastlingStatus::Unavailable;
                    new_move.castling_rights.white_queenside = CastlingStatus::Unavailable;
                }
                Color::Black => {
                    new_move.castling_rights.black_kingside = CastlingStatus::Unavailable;
                    new_move.castling_rights.black_queenside = CastlingStatus::Unavailable;
                }
            }
        } else if matches!(self.get_piece(new_move.dest), Some(Kind::Rook(_))) {
            match (self.current_turn, new_move.start) {
                (Color::White, Square { rank: 0, file: 0 }) => {
                    new_move.castling_rights.white_queenside = CastlingStatus::Unavailable;
                }
                (Color::White, Square { rank: 0, file: 7 }) => {
                    new_move.castling_rights.white_kingside = CastlingStatus::Unavailable;
                }
                (Color::Black, Square { rank: 7, file: 0 }) => {
                    new_move.castling_rights.black_queenside = CastlingStatus::Unavailable;
                }
                (Color::Black, Square { rank: 7, file: 7 }) => {
                    new_move.castling_rights.black_kingside = CastlingStatus::Unavailable;
                }
                _ => (),
            }
        }

        if let Some(piece) = new_move.captured_piece {
            if matches!(piece, Kind::Rook(_)) {
                match (self.current_turn, new_move.dest) {
                    (Color::White, Square { rank: 7, file: 0 }) => {
                        new_move.castling_rights.black_queenside = CastlingStatus::Unavailable;
                    }
                    (Color::White, Square { rank: 7, file: 7 }) => {
                        new_move.castling_rights.black_kingside = CastlingStatus::Unavailable;
                    }
                    (Color::Black, Square { rank: 0, file: 0 }) => {
                        new_move.castling_rights.white_queenside = CastlingStatus::Unavailable;
                    }
                    (Color::Black, Square { rank: 0, file: 7 }) => {
                        new_move.castling_rights.white_kingside = CastlingStatus::Unavailable;
                    }
                    _ => (),
                }
            }
        }
    }

    /// Unmakes a half-move on this board
    ///
    /// # Arguments
    ///
    /// * `old_move` - A Ply that holds the origin and destination square of the move.
    ///
    /// # Panics
    /// Will panic if there is no piece at the destination square.
    ///
    /// # Examples
    /// ```
    /// ```
    #[allow(dead_code)]
    pub fn unmake_move(&mut self) {
        let old_move = self
            .history
            .pop()
            .expect("No previous move in the board history!");

        self.replace_square(old_move.dest, old_move.start);

        if let Some(promoted_piece) = old_move.promoted_to {
            self.remove_piece(old_move.start, promoted_piece);
            self.add_piece(old_move.start, Kind::Pawn(self.current_turn.opposite()));
        }

        if let Some(captured_piece) = old_move.captured_piece {
            if old_move.en_passant {
                self.add_piece(
                    Square {
                        file: old_move.dest.file,
                        rank: old_move.start.rank,
                    },
                    captured_piece,
                );
            } else {
                self.add_piece(old_move.dest, captured_piece);
            }
        }

        if old_move.is_castles {
            let (rook_start, rook_dest) = match old_move.dest {
                Square { rank: 0, file: 6 } => (Square::from("h1"), Square::from("f1")),
                Square { rank: 0, file: 2 } => (Square::from("a1"), Square::from("d1")),
                Square { rank: 7, file: 6 } => (Square::from("h8"), Square::from("f8")),
                Square { rank: 7, file: 2 } => (Square::from("a8"), Square::from("d8")),
                _ => panic!("Invalid castling king destination {}", old_move.dest),
            };

            self.replace_square(rook_dest, rook_start);
        }

        if self.history.last().is_some_and(|f| f.is_double_pawn_push) {
            self.en_passant_file = Some(self.history.last().unwrap().dest.file);
        } else {
            self.en_passant_file = None;
        }

        if self.current_turn == Color::White {
            self.fullmove_counter -= 1;
        }

        // Cannot make a move if game is over, so all previous moves are in progress
        self.game_state = GameState::InProgress;

        self.switch_turn();
    }
}

impl fmt::Display for Board {
    /// Prints out a symbolic representation of the board in an 8x8 grid.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                if let Some(piece) = self.get_piece(Square { rank: i, file: j }) {
                    write!(f, "{piece}")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::utils::tests::check_unique_equality;
    use boardbuilder::BoardBuilder;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_board() {
        let board = Board::default();
        assert_eq!(board.current_turn, Color::White);
        assert_eq!(board.en_passant_file, None);
        assert_eq!(board.game_state, GameState::InProgress);
        assert_eq!(board.history, vec![Ply::default()]);
    }

    #[test]
    fn test_castle_status() {
        let board = BoardBuilder::default()
            .castling(CastlingKind::BlackKingside, CastlingStatus::Available)
            .castling(CastlingKind::BlackQueenside, CastlingStatus::Unavailable)
            .castling(CastlingKind::WhiteKingside, CastlingStatus::Unavailable)
            .castling(CastlingKind::WhiteQueenside, CastlingStatus::Available)
            .build();

        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_castling_ability() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Available)
        );
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Available)
        );

        board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Unavailable)
        );
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("r4rk1/8/8/8/8/8/8/R3K2R w KQq - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Unavailable)
        );
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Available)
        );

        board = Board::from_fen("1r3rk1/8/8/8/8/8/8/R3K2R w KQq - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Available)
        );

        board = Board::from_fen("2r2rk1/8/8/8/8/8/8/R3K2R w KQq - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("3r1rk1/8/8/8/8/8/8/R3K2R w KQq - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("4rrk1/8/8/8/8/8/8/R3K2R w KQq - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("2kr3r/8/8/8/8/8/8/R3K2R w KQk - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteQueenside),
            Ok(CastlingStatus::Unavailable)
        );
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Available)
        );

        board = Board::from_fen("2kr2r1/8/8/8/8/8/8/R3K2R w KQk - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("2kr1r2/8/8/8/8/8/8/R3K2R w KQk - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Unavailable)
        );

        board = Board::from_fen("2krr3/8/8/8/8/8/8/R3K2R w KQk - 0 1");
        assert_eq!(
            board.castling_ability(CastlingKind::WhiteKingside),
            Ok(CastlingStatus::Unavailable)
        );
    }

    #[test]
    fn test_no_pieces_between_castling_white() {
        let mut board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteKingside)
            .is_ok());
        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteQueenside)
            .is_ok());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White))
            .piece(Square::from("g1"), Kind::Rook(Color::White))
            .piece(Square::from("b1"), Kind::Rook(Color::White))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White))
            .piece(Square::from("g1"), Kind::Bishop(Color::Black))
            .piece(Square::from("b1"), Kind::Bishop(Color::Black))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White))
            .piece(Square::from("f1"), Kind::Bishop(Color::White))
            .piece(Square::from("c1"), Kind::Bishop(Color::White))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White))
            .piece(Square::from("d1"), Kind::Bishop(Color::White))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteKingside)
            .is_ok());
        assert!(board
            .no_pieces_between_castling(CastlingKind::WhiteQueenside)
            .is_err());
    }

    #[test]
    fn test_no_pieces_between_castling_black() {
        let mut board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackKingside)
            .is_ok());
        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackQueenside)
            .is_ok());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black))
            .piece(Square::from("g8"), Kind::Rook(Color::Black))
            .piece(Square::from("b8"), Kind::Rook(Color::Black))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black))
            .piece(Square::from("g8"), Kind::Bishop(Color::White))
            .piece(Square::from("b8"), Kind::Bishop(Color::White))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black))
            .piece(Square::from("f8"), Kind::Bishop(Color::Black))
            .piece(Square::from("c8"), Kind::Bishop(Color::Black))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackKingside)
            .is_err());
        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackQueenside)
            .is_err());

        board = BoardBuilder::construct_empty_board()
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black))
            .piece(Square::from("d8"), Kind::Bishop(Color::Black))
            .build();

        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackKingside)
            .is_ok());
        assert!(board
            .no_pieces_between_castling(CastlingKind::BlackQueenside)
            .is_err());
    }

    #[test]
    fn test_no_checks_castling_white() {
        let builder = BoardBuilder::construct_empty_board()
            .turn(Color::White)
            .piece(Square::from("a1"), Kind::Rook(Color::White))
            .piece(Square::from("e1"), Kind::King(Color::White))
            .piece(Square::from("h1"), Kind::Rook(Color::White));

        let test_cases = [
            (Square::from("a8"), Ok(()), Ok(())),
            (Square::from("b8"), Ok(()), Ok(())),
            (Square::from("c8"), Ok(()), Err(())),
            (Square::from("d8"), Ok(()), Err(())),
            (Square::from("e8"), Err(()), Err(())),
            (Square::from("f8"), Err(()), Ok(())),
            (Square::from("g8"), Err(()), Ok(())),
            (Square::from("h8"), Ok(()), Ok(())),
        ];

        for (square, kingside, queenside) in test_cases {
            println!("Testing square {}", square);
            let board = builder
                .clone()
                .piece(square, Kind::Rook(Color::Black))
                .build();

            match kingside {
                Ok(_) => assert!(board
                    .no_checks_castling(CastlingKind::WhiteKingside)
                    .is_ok()),
                Err(_) => assert!(board
                    .no_checks_castling(CastlingKind::WhiteKingside)
                    .is_err()),
            };

            match queenside {
                Ok(_) => assert!(board
                    .no_checks_castling(CastlingKind::WhiteQueenside)
                    .is_ok()),
                Err(_) => assert!(board
                    .no_checks_castling(CastlingKind::WhiteQueenside)
                    .is_err()),
            };
        }
    }

    #[test]
    fn test_get_attacked_squares_starting_board() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.get_attacked_squares(Color::White),
            Bitboard::new(0b0111111011111111111111110000000000000000000000000000000000000000)
        );
        assert_eq!(
            board.get_attacked_squares(Color::Black),
            Bitboard::new(0b0000000000000000000000000000000000000000111111111111111101111110)
        );
    }

    #[test]
    fn test_get_attacked_squares_position_1() {
        let board = Board::from_fen("r3kb1r/p2bqpp1/5n2/4Q1p1/3P4/8/PPP2PPP/RNB1K2R b KQkq - 0 13");
        assert_eq!(
            board.get_attacked_squares(Color::White),
            Bitboard::new(0b0111111011111001111111101011111011110011100000011000000000000000)
        );
        assert_eq!(
            board.get_attacked_squares(Color::Black),
            Bitboard::new(0b0000001000010100001110000111111100111000111111111011101101111010)
        );
    }

    #[test]
    fn test_get_attacked_squares_position_2() {
        let board =
            Board::from_fen("r1bqkbnr/1p2pppp/p2p4/3Pn3/4PB2/8/PP3PPP/RN1QKBNR w KQkq - 1 7");
        assert_eq!(
            board.get_attacked_squares(Color::White),
            Bitboard::new(0b0111111011111111111111110011011101000100101010000000000000000000)
        );
        assert_eq!(
            board.get_attacked_squares(Color::Black),
            Bitboard::new(0b0000000000000000100101011111101001001101111111111111110101111110)
        );
    }

    #[test]
    fn test_get_attacked_squares_position_3() {
        let board = Board::from_fen("2r1kb1r/1p1bpppp/pq6/3PB3/8/2N5/PPQ2PPP/R3KB1R b KQk - 4 13");
        assert_eq!(
            board.get_attacked_squares(Color::White),
            Bitboard::new(0b0111111111111111111111010010011101001111100101100010001000000000)
        );
        assert_eq!(
            board.get_attacked_squares(Color::Black),
            Bitboard::new(0b0000001011000100011111010010101000111101111111111111101101111110)
        );
    }

    #[test]
    fn test_no_checks_castling_black() {
        let builder = BoardBuilder::construct_empty_board()
            .turn(Color::Black)
            .piece(Square::from("a8"), Kind::Rook(Color::Black))
            .piece(Square::from("e8"), Kind::King(Color::Black))
            .piece(Square::from("h8"), Kind::Rook(Color::Black));

        let test_cases = [
            (Square::from("a1"), Ok(()), Ok(())),
            (Square::from("b1"), Ok(()), Ok(())),
            (Square::from("c1"), Ok(()), Err(())),
            (Square::from("d1"), Ok(()), Err(())),
            (Square::from("e1"), Err(()), Err(())),
            (Square::from("f1"), Err(()), Ok(())),
            (Square::from("g1"), Err(()), Ok(())),
            (Square::from("h1"), Ok(()), Ok(())),
        ];

        for (square, kingside, queenside) in test_cases {
            println!("Testing square {}", square);
            let board = builder
                .clone()
                .piece(square, Kind::Rook(Color::White))
                .build();

            match kingside {
                Ok(_) => assert!(board
                    .no_checks_castling(CastlingKind::BlackKingside)
                    .is_ok()),
                Err(_) => assert!(board
                    .no_checks_castling(CastlingKind::BlackKingside)
                    .is_err()),
            };

            match queenside {
                Ok(_) => assert!(board
                    .no_checks_castling(CastlingKind::BlackQueenside)
                    .is_ok()),
                Err(_) => assert!(board
                    .no_checks_castling(CastlingKind::BlackQueenside)
                    .is_err()),
            };
        }
    }

    #[test]
    fn test_get_piece1() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.get_piece(Square::from("a1")).unwrap(),
            Kind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.get_piece(Square::from("h8")).unwrap(),
            Kind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.get_piece(Square::from("h7")).unwrap(),
            Kind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = BoardBuilder::construct_starting_board().build();
        assert!(board.get_piece(Square::from("e5")).is_none());
    }

    #[test]
    #[should_panic = "attempt to shift left with overflow"]
    fn test_get_piece_ooblack_rank() {
        let board = BoardBuilder::construct_starting_board().build();
        board.get_piece(Square { rank: 8, file: 7 }).unwrap();
    }

    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value"]
    fn test_get_piece_ooblack_file() {
        let board = BoardBuilder::construct_starting_board().build();
        board.get_piece(Square { rank: 0, file: 8 }).unwrap();
    }

    #[test]
    fn test_get_all_moves() {
        let board = BoardBuilder::construct_starting_board().build();
        let all_moves = board.get_all_moves();

        assert!(!all_moves.is_empty());
    }

    #[test]
    fn test_add_piece() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let square = Square::from("a3");
        board.add_piece(square, Kind::Queen(Color::White));
        assert_eq!(board.get_piece(square).unwrap(), Kind::Queen(Color::White));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let square = Square::from("a2");

        // Should do nothing, since there is a white pawn here, not a black pawn
        board.remove_piece(square, Kind::Pawn(Color::Black));
        assert_eq!(board.get_piece(square).unwrap(), Kind::Pawn(Color::White));

        board.remove_piece(square, Kind::Pawn(Color::White));
        assert!(board.get_piece(square).is_none());
    }

    #[test]
    fn test_board_display() {
        let board = BoardBuilder::construct_starting_board().build();
        let correct =
            "♖♘♗♕♔♗♘♖\n♙♙♙♙♙♙♙♙\n--------\n--------\n--------\n--------\n♟♟♟♟♟♟♟♟\n♜♞♝♛♚♝♞♜\n";
        assert_eq!(board.to_string(), correct);
    }

    #[test]
    fn test_is_white_turn() {
        let board = BoardBuilder::construct_starting_board().build();
        assert!(board.current_turn == Color::White);
    }

    #[test]
    fn test_is_black_turn() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        assert!(board.current_turn == Color::Black);
    }

    #[test]
    fn test_kingside_castle_true() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_queenside_castle_true() {
        let board = BoardBuilder::construct_starting_board().build();
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_kingside_castle_false_white() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qkq - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_kingside_castle_false_black() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQq - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_kingside_castle_false_both() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
    }

    #[test]
    fn test_queenside_castle_false_white() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kkq - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_queenside_castle_false_black() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );
    }

    #[test]
    fn test_queenside_castle_false_both() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kk - 0 1");
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );
    }

    #[test]
    fn test_castle_make_unmake_move() {
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");

        let moves_1 = board.get_legal_moves();
        let white_queenside_castle_move = moves_1
            .clone()
            .into_iter()
            .find(|mv| mv.is_castles && mv.dest.file == 2);
        assert!(white_queenside_castle_move.is_some());
        board.make_move(white_queenside_castle_move.unwrap());
        assert_eq!(
            board
                .get_piece(Square::from("c1"))
                .expect("No King found at c1!"),
            Kind::King(Color::White)
        );
        assert_eq!(
            board
                .get_piece(Square::from("d1"))
                .expect("No Rook found at d1!"),
            Kind::Rook(Color::White)
        );

        let moves_2 = board.get_legal_moves();
        let black_pawn_move = moves_2
            .clone()
            .into_iter()
            .find(|mv| mv.start.file == 0 && mv.dest.rank == 5);
        assert!(black_pawn_move.is_some());
        board.make_move(black_pawn_move.unwrap());
        assert_eq!(board.get_piece(Square::from("a7")), None);
        assert_eq!(
            board
                .get_piece(Square::from("a6"))
                .expect("No pawn found at a6!"),
            Kind::Pawn(Color::Black)
        );

        board.unmake_move();
        let moves_3 = board.get_legal_moves();
        check_unique_equality(moves_2.clone(), moves_3.clone());
        let black_queenside_castle_move = moves_3
            .into_iter()
            .find(|mv| mv.is_castles && mv.dest.file == 2);
        assert!(black_queenside_castle_move.is_some());
        board.make_move(black_queenside_castle_move.unwrap());
        assert_eq!(
            board
                .get_piece(Square::from("c8"))
                .expect("No King found at c8!"),
            Kind::King(Color::Black)
        );
        assert_eq!(
            board
                .get_piece(Square::from("d8"))
                .expect("No Rook found at d8!"),
            Kind::Rook(Color::Black)
        );

        board.unmake_move();
        check_unique_equality(moves_2, board.get_legal_moves());
        board.unmake_move();

        let moves_4 = board.get_legal_moves();
        check_unique_equality(moves_1.clone(), moves_4.clone());
        let white_pawn_move = moves_4
            .clone()
            .into_iter()
            .find(|mv| mv.start.file == 0 && mv.dest.rank == 2);
        assert!(white_pawn_move.is_some());
        board.make_move(white_pawn_move.unwrap());
        assert_eq!(board.get_piece(Square::from("a2")), None);
        assert_eq!(
            board
                .get_piece(Square::from("a3"))
                .expect("No pawn found at a3!"),
            Kind::Pawn(Color::White)
        );

        board.unmake_move();
        check_unique_equality(moves_1, board.get_legal_moves());
    }

    #[test]
    fn test_make_unmake_move_single() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let start = Square::from("a2");
        let dest = Square::from("a3");
        let ply = Ply::new(start, dest);

        assert_eq!(board.current_turn, Color::White);

        assert!(board.get_piece(dest).is_none());
        board.make_move(ply);
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.current_turn, Color::Black);

        assert!(board.get_piece(start).is_none());

        board.unmake_move();
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.current_turn, Color::White);

        assert!(board.get_piece(dest).is_none());
    }

    #[test]
    fn test_make_unmake_move_double() {
        // Make and unmake two moves in a row
        let mut board = BoardBuilder::construct_starting_board().build();
        let ply1 = Ply::new(Square::from("e2"), Square::from("e4"));
        let ply2 = Ply::new(Square::from("e7"), Square::from("e5"));

        assert_eq!(board.current_turn, Color::White);

        assert!(board.get_piece(ply1.dest).is_none());
        assert!(board.get_piece(ply2.dest).is_none());
        board.make_move(ply1);
        assert_eq!(
            board.get_piece(ply1.dest).unwrap(),
            Kind::Pawn(Color::White)
        );
        assert!(board.get_piece(ply1.start).is_none());
        assert!(board.get_piece(ply2.dest).is_none());
        assert_eq!(board.current_turn, Color::Black);

        board.make_move(ply2);
        assert_eq!(
            board.get_piece(ply2.dest).unwrap(),
            Kind::Pawn(Color::Black)
        );
        assert!(board.get_piece(ply2.start).is_none());
        assert!(board.get_piece(ply1.start).is_none());
        assert_eq!(board.current_turn, Color::White);

        board.unmake_move();
        assert_eq!(
            board.get_piece(ply2.start).unwrap(),
            Kind::Pawn(Color::Black)
        );
        assert!(board.get_piece(ply2.dest).is_none());
        assert!(board.get_piece(ply1.start).is_none());
        assert_eq!(board.current_turn, Color::Black);

        board.unmake_move();
        assert_eq!(
            board.get_piece(ply1.start).unwrap(),
            Kind::Pawn(Color::White)
        );
        assert!(board.get_piece(ply1.dest).is_none());
        assert!(board.get_piece(ply2.dest).is_none());
        assert_eq!(board.current_turn, Color::White);
    }

    #[test]
    fn test_make_unmake_move_capture() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let start = Square::from("a2"); // White Pawn
        let dest = Square::from("a7"); // Black Pawn
        let ply = Ply::builder(start, dest)
            .captured(Kind::Pawn(Color::Black))
            .build();
        assert_eq!(board.current_turn, Color::White);

        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::Black));
        board.make_move(ply);
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(start).is_none());
        assert_eq!(board.current_turn, Color::Black);

        board.unmake_move();
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::Black));
        assert_eq!(board.current_turn, Color::White);
    }

    #[test]
    fn test_castling_capture_rook() {
        let mut board =
            Board::from_fen("r3k2r/pppppppp/1N4N1/8/8/1n4n1/PPPPPPPP/R3K2R w KQkq - 0 1");

        let ply_capture_black_kingside_rook = Ply::builder(Square::from("g6"), Square::from("h8"))
            .captured(Kind::Rook(Color::Black))
            .build();
        let ply_capture_black_queenside_rook = Ply::builder(Square::from("b6"), Square::from("a8"))
            .captured(Kind::Rook(Color::Black))
            .build();
        let ply_capture_white_kingside_rook = Ply::builder(Square::from("g3"), Square::from("h1"))
            .captured(Kind::Rook(Color::White))
            .build();
        let ply_capture_white_queenside_rook = Ply::builder(Square::from("b3"), Square::from("a1"))
            .captured(Kind::Rook(Color::White))
            .build();

        board.make_move(ply_capture_black_kingside_rook);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );

        board.make_move(ply_capture_white_kingside_rook);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );

        board.unmake_move();
        board.unmake_move();

        board.make_move(ply_capture_black_queenside_rook);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );

        board.make_move(ply_capture_white_queenside_rook);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );

        board.unmake_move();
        board.unmake_move();

        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_castling_move_rook() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        let ply_h1 = Ply::new(Square::from("h1"), Square::from("h2"));
        board.make_move(ply_h1);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );

        let ply_h8 = Ply::new(Square::from("h8"), Square::from("h7"));
        board.make_move(ply_h8);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
        board.unmake_move();
        board.unmake_move();

        let ply_a1 = Ply::new(Square::from("a1"), Square::from("a2"));
        board.make_move(ply_a1);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );

        let ply_a8 = Ply::new(Square::from("a8"), Square::from("a7"));
        board.make_move(ply_a8);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        board.unmake_move();
        board.unmake_move();
    }

    #[test]
    fn test_castling_move_king() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        let ply_e1 = Ply::new(Square::from("e1"), Square::from("e2"));
        board.make_move(ply_e1);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );

        let ply_e8 = Ply::new(Square::from("e8"), Square::from("e7"));
        board.make_move(ply_e8);
        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Unavailable
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Unavailable
        );
        board.unmake_move();
        board.unmake_move();

        assert_eq!(
            board.castle_status(CastlingKind::WhiteKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackKingside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::WhiteQueenside),
            CastlingStatus::Available
        );
        assert_eq!(
            board.castle_status(CastlingKind::BlackQueenside),
            CastlingStatus::Available
        );
    }

    #[test]
    fn test_make_unmake_move_promotion() {
        let mut board = Board::from_fen("8/5P2/2k5/8/4K3/8/8/8 w - - 0 1");
        let start = Square::from("f7"); // White Pawn
        let dest = Square::from("f8");
        let ply = Ply::builder(start, dest)
            .promoted_to(Kind::Queen(Color::White))
            .build();

        assert_eq!(board.current_turn, Color::White);
        assert_eq!(board.get_piece(start), Some(Kind::Pawn(Color::White)));
        assert_eq!(board.get_piece(dest), None);

        dbg!(ply);
        board.make_move(ply);
        assert_eq!(board.get_piece(dest), Some(Kind::Queen(Color::White)));
        assert!(board.get_piece(start).is_none());
        assert_eq!(board.current_turn, Color::Black);

        board.unmake_move();
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(dest).is_none());
        assert_eq!(board.current_turn, Color::White);
    }

    #[test]
    fn test_make_unmake_move_promotion_capture() {
        let mut board = Board::from_fen("6n1/5P2/2k5/8/4K3/8/8/8 w - - 0 1");
        let start = Square::from("f7"); // White Pawn
        let dest = Square::from("g8"); // Black Knight
        let ply = Ply::builder(start, dest)
            .captured(Kind::Knight(Color::Black))
            .promoted_to(Kind::Queen(Color::White))
            .build();

        assert_eq!(board.current_turn, Color::White);
        assert_eq!(board.get_piece(start), Some(Kind::Pawn(Color::White)));
        assert_eq!(board.get_piece(dest), Some(Kind::Knight(Color::Black)));

        board.make_move(ply);
        assert_eq!(board.get_piece(dest), Some(Kind::Queen(Color::White)));
        assert!(board.get_piece(start).is_none());
        assert_eq!(board.current_turn, Color::Black);

        board.unmake_move();
        assert_eq!(board.get_piece(start), Some(Kind::Pawn(Color::White)));
        assert_eq!(board.get_piece(dest), Some(Kind::Knight(Color::Black)));
        assert_eq!(board.current_turn, Color::White);
    }

    #[test]
    fn test_is_not_in_check() {
        let board = BoardBuilder::construct_starting_board().build();
        assert!(!board.is_in_check(Color::White));
    }

    #[test]
    fn test_is_in_check_white_by_queen() {
        let board = Board::from_fen("8/1k6/2q5/8/8/2K3Q1/8/8 w - - 0 1");
        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn test_is_in_check_black_by_queen() {
        let board = Board::from_fen("8/1K6/2Q5/8/8/2k3q1/8/8 b - - 0 1");
        assert!(board.is_in_check(Color::Black));
    }

    #[test]
    fn test_set_game_state() {
        let mut board = BoardBuilder::construct_starting_board().build();
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::InProgress);

        board = Board::from_fen("7k/8/7K/4N3/6P1/1B3P2/P7/8 b - - 4 72"); // Stalemate
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::Stalemate);

        board = Board::from_fen("7k/8/6KP/8/8/8/8/8 w - - 100 1"); // FiftyMoveRule
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::FiftyMoveRule);

        board = Board::from_fen("4r1k1/6b1/p7/1pQ5/8/8/PPP2PPP/3q2K1 w - - 0 34"); // Checkmate, Black wins
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::CheckmateWhite);

        board = Board::from_fen("Q7/8/8/3P4/3Q1K2/kP6/P7/8 b - - 3 65"); // Checkmate, White wins
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::CheckmateBlack);
    }

    #[test]
    fn test_get_winner() {
        let mut board = Board::from_fen("4r1k1/6b1/p7/1pQ5/8/8/PPP2PPP/3q2K1 w - - 0 34"); // Checkmate, Black wins
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::CheckmateWhite);
        assert_eq!(board.get_winner(), Some(Color::Black));

        board = Board::from_fen("Q7/8/8/3P4/3Q1K2/kP6/P7/8 b - - 3 65"); // Checkmate, White wins
        assert_eq!(board.game_state, GameState::Unknown);
        board.set_game_state();
        assert_eq!(board.game_state, GameState::CheckmateBlack);
        assert_eq!(board.get_winner(), Some(Color::White));
    }

    #[test]
    fn test_find_move() {
        let mut board = BoardBuilder::construct_starting_board().build();

        let notation_exists = "a2a4";
        let notation_made_up = "a2a5";
        assert!(board.find_move(notation_exists).is_ok());
        assert!(board.find_move(notation_made_up).is_err());
    }

    #[test]
    fn test_is_game_over() {
        let mut board = BoardBuilder::construct_starting_board().build();
        assert!(!board.is_game_over());

        let tests = [
            (GameState::InProgress, false),
            (GameState::CheckmateWhite, true),
            (GameState::CheckmateBlack, true),
            (GameState::Stalemate, true),
            (GameState::FiftyMoveRule, true),
            (GameState::ThreefoldRepetition, true),
        ];

        for (state, correct) in tests.iter() {
            board.game_state = state.clone();
            assert_eq!(board.is_game_over(), *correct);
        }
    }

    #[test]
    fn test_get_legal_moves_count_start() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let result = board.get_legal_moves().len();
        let correct = 20;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_1() {
        let mut board = Board::from_fen("2k1b3/8/8/8/2K5/5Q2/5PPP/5RN1 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 39;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_2() {
        let mut board = Board::from_fen("8/1K6/2Q5/8/8/6q1/2k5/8 b - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_3() {
        let mut board = Board::from_fen("8/1K6/2Q5/8/8/6q1/2k5/8 b - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_4() {
        let mut board = Board::from_fen("8/1k6/2q5/5b2/8/R5Q1/2K5/3N4 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 3;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_5() {
        let mut board = Board::from_fen("8/1k6/2q5/8/8/R5Q1/2K5/3N4 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 8;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_6() {
        let mut board = Board::from_fen("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 50;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_7() {
        let mut board = Board::from_fen("rnbqkbnr/8/5B2/Q2B4/3R2N1/2N5/1K6/7R w kq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 72;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_8() {
        let mut board = Board::from_fen("5b2/r7/1qn2B1n/1Q6/3R2N1/2N3k1/1K2Br2/3b3R w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 44;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_9() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 26;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_10() {
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 25;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_11() {
        let mut board =
            Board::from_fen("4r2k/4qpRb/2p1p2Q/1p3r1P/p2P4/P4P2/1PP1N3/1K4R1 b - - 2 32");
        let result = board.get_legal_moves().len();
        let correct = 31;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_12() {
        let mut board =
            Board::from_fen("r2qk2r/pp3ppb/2p1pn1p/4Q2P/2B5/3P2N1/PPP2PP1/R3K2R b KQkq - 0 14");
        let result = board.get_legal_moves().len();
        let correct = 37;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_13() {
        let mut board =
            Board::from_fen("r2q1rk1/pp3ppb/2p1pn1p/4Q2P/2B5/3P2N1/PPP2PP1/2KR3R b - - 2 15");

        let result = board.get_legal_moves().len();
        let correct = 33;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_14() {
        let mut board = Board::from_fen("8/6P1/8/2k5/8/5K2/8/8 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 12;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_15() {
        let mut board = Board::from_fen("8/1K6/8/8/5k2/8/6p1/5B2 b - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 16;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_16() {
        let mut board = Board::from_fen("8/p1KP1p2/5rkp/8/8/8/8/3R4 w - - 0 46");
        let result = board.get_legal_moves().len();
        let correct = 20;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_17() {
        let mut board = Board::from_fen("8/p1KPrp2/6kp/8/8/8/8/3R4 w - - 0 46");
        let result = board.get_legal_moves().len();
        let correct = 18;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_18() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp2ppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1");
        let result = board.get_legal_moves().len();
        let correct = 31;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_19() {
        let mut board = Board::from_fen("1k6/8/8/4Pp2/1K6/8/8/8 w - f6 0 1");
        let result = board.get_legal_moves().len();
        let correct = 10;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_20() {
        let mut board =
            Board::from_fen("3r1rk1/pp1qBpbp/6p1/3p4/3P4/5Q1P/PPP2PP1/R3R1K1 b - - 0 16");
        let result = board.get_legal_moves().len();
        let correct = 32;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_21() {
        let mut board =
            Board::from_fen("r3k2r/pbppqNb1/1n2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/1R2K2R b Kkq - 2 2");
        let result = board.get_legal_moves().len();
        let correct = 44;

        assert_eq!(result, correct);
    }
}
