use std::fmt;

pub mod bitboard;
mod bitboards;
mod boardbuilder;
pub mod piece;
pub mod ply;
pub mod serialize;
pub mod square;

use bitboard::Bitboard;
use bitboards::Bitboards;
use boardbuilder::BoardBuilder;
use piece::{Color, Kind};
pub use ply::Ply;
use square::Square;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastlingStatus {
    #[default]
    Availiable,
    Unavailiable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastlingKind {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub current_turn: Color,
    pub halfmove_clock: u8,
    pub fullmove_counter: u16,

    white_kingside_castling: CastlingStatus,
    white_queenside_castling: CastlingStatus,
    black_kingside_castling: CastlingStatus,
    black_queenside_castling: CastlingStatus,

    en_passant_file: Option<u8>,

    bitboards: Bitboards,

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
            halfmove_clock: 0,
            fullmove_counter: 1,

            white_kingside_castling: CastlingStatus::Availiable,
            white_queenside_castling: CastlingStatus::Availiable,
            black_kingside_castling: CastlingStatus::Availiable,
            black_queenside_castling: CastlingStatus::Availiable,

            bitboards: Bitboards::default(),

            en_passant_file: None,

            history: Vec::new(),
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
    #[allow(dead_code)]
    pub const fn builder() -> BoardBuilder {
        BoardBuilder::default()
    }

    /// Creates a new board object that represents the starting board state in a normal game
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// ```
    pub fn construct_starting_board() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    /// Creates a new board object without any pieces on the board
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_empty_board();
    /// ```
    pub fn construct_empty_board() -> Self {
        Self {
            bitboards: Bitboards::new(),
            ..Self::default()
        }
    }

    /// Returns a boolean representing whether or not the current player has kingside castling rights
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert_eq!(board.kingside_castle_status(), Castling::Availiable);
    /// ```
    #[allow(dead_code)]
    pub const fn kingside_castle_status(&self) -> CastlingStatus {
        match self.current_turn {
            Color::White => self.white_kingside_castling,
            Color::Black => self.black_kingside_castling,
        }
    }

    /// Returns a boolean representing whether or not the current player has queenside castling rights
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert_eq!(board.queenside_castle_status(), Castling::Availiable);
    /// ```
    #[allow(dead_code)]
    pub const fn queenside_castle_status(&self) -> CastlingStatus {
        match self.current_turn {
            Color::White => self.white_queenside_castling,
            Color::Black => self.black_queenside_castling,
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
    /// let board = Board::construct_starting_board();
    /// assert_eq!(PieceKind::Rook(Color::White), board.get_piece(Square::new("a1")));
    /// assert_eq!(None, board.get_piece(Square::new("b3")));
    /// ```
    pub fn get_piece(&self, square: Square) -> Option<Kind> {
        self.bitboards.get_piece_kind(square)
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
    /// let board = Board::construct_starting_board();
    /// let movelist = board.get_all_moves(Square::new("a2"));
    /// ```
    fn get_all_moves(&self) -> Vec<Ply> {
        let mut all_moves = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                let square = Square { rank: i, file: j };
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
                                        rank: square.rank,
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
        }

        all_moves
    }

    /// Returns a list of all legal moves for the current side
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let movelist = board.get_all_moves(Square::new("a2"));
    /// ```
    pub fn get_legal_moves(&mut self) -> Vec<Ply> {
        self.filter_moves(self.get_all_moves())
    }

    /// Filters a list of moves to only include legal moves
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board()
    /// let movelist = board.get_all_moves(Square::new("e1"));
    /// let legal_moves = filter_moves(&board, movelist);
    /// ```
    fn filter_moves(&mut self, moves: Vec<Ply>) -> Vec<Ply> {
        moves
            .into_iter()
            .filter(|mv| self.is_legal_move(*mv).is_ok())
            .collect()
    }

    /// Returns a boolean representing whether or not a given move is legal
    ///
    /// # Examples
    /// ```
    /// let ply = Ply(Square::new("e2"), Square::new("e9"));
    /// assert!(!board.is_legal_move(ply));
    /// ```
    fn is_legal_move(&mut self, ply: Ply) -> Result<Ply, &'static str> {
        self.is_on_board(ply)
            .and_then(|_| self.is_self_capture(ply))
            .and_then(|_| self.is_illegal_jump(ply))
            .and_then(|_| self.is_illegal_pawn_move(ply))
            .and_then(|_| self.is_illegal_castling(ply))?;

        // Don't allow leaving your king in check
        self.make_move(ply);
        let result = self.is_in_check();
        self.unmake_move();
        if result {
            return Err("Move is not valid. The move would leave the king in check.");
        }

        Ok(ply)
    }

    /// Returns a boolean representing whether or not a given move is on the constraits of the board.
    ///
    /// Checks if the move start and destination are within the bounds of the board, that the move's start and destination are not the same, and that the start square contains a piece.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let ply1 = Ply(Square::new("e2"), Square::new("e9"));
    /// let ply2 = Ply(Square::new("e2"), Square::new("e2"));
    /// let ply3 = Ply(Square::new("e2"), Square::new("e4"));
    /// assert!(!board.is_on_board(ply1));
    /// assert!(!board.is_on_board(ply2));
    /// assert!(board.is_on_board(ply3));
    /// ```
    fn is_on_board(&self, ply: Ply) -> Result<Ply, &'static str> {
        match ply {
            Ply { start, .. } if start.rank >= 8 => {
                Err("Move is not valid. The start square rank is off the board.")
            }
            Ply { start, .. } if start.file >= 8 => {
                Err("Move is not valid. The start square file is off the board.")
            }
            Ply { dest, .. } if dest.rank >= 8 => {
                Err("Move is not valid. The dest square rank is off the board.")
            }
            Ply { dest, .. } if dest.file >= 8 => {
                Err("Move is not valid. The dest square file is off the board.")
            }
            Ply { start, dest, .. } if start == dest => {
                Err("Move is not valid. The start and destination squares are the same.")
            }
            Ply { start, .. } if self.get_piece(start).is_none() => {
                Err("Move is not valid. The start square is empty.")
            }
            _ => Ok(ply),
        }
    }

    /// Returns a boolean representing whether or not a given move is a self-capture
    ///
    /// A self capture is defined as a move that captures a piece of the same color as the moving piece.
    ///
    /// # Panics
    ///
    /// Will panic if there is no piece at the start square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let ply1 = Ply(Square::new("e2"), Square::new("e4"));
    /// let ply1 = Ply(Square::new("e2"), Square::new("d2"));
    /// assert!(!board.is_self_capture(ply1));
    /// assert!(board.is_self_capture(ply2));
    /// ```
    fn is_self_capture(&self, ply: Ply) -> Result<Ply, &'static str> {
        let dest_piece = self.get_piece(ply.dest);
        if dest_piece
            .is_some_and(|pc| pc.get_color() == self.get_piece(ply.start).unwrap().get_color())
        {
            Err("Move is not valid. The move would capture a piece of the same color.")
        } else {
            Ok(ply)
        }
    }

    /// Returns a boolean representing whether or not a given move jumps through other pieces without being a knight.
    ///
    /// # Panics
    ///
    /// Will panic if there is no piece at the start square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let ply1 = Ply(Square::new("e2"), Square::new("e4"));
    /// let ply2 = Ply(Square::new("e1"), Square::new("e3"));
    /// assert!(!board.is_illegal_jump(ply1));
    /// assert!(board.is_illegal_jump(ply2));
    /// ```
    fn is_illegal_jump(&self, ply: Ply) -> Result<Ply, &'static str> {
        // Castling needs more spaces clear than between the start and dest squares
        if ply.is_castles {
            return Ok(ply);
        }

        match self.get_piece(ply.start).unwrap() {
            Kind::Knight(_c) => Ok(ply),
            _ => self.no_pieces_between(ply.start, ply.dest).map_or(
                Err("Move is not valid. The move jumps through other pieces."),
                |()| Ok(ply),
            ),
        }
    }

    /// Returns a boolean representing whether or not a given move is an illegal pawn move
    ///
    /// Checks if a pawn is capturing forward, moving diagonally without
    /// capturing, or is performing an en passant when it is not allowed.
    ///
    /// # Assumptions
    ///
    /// Assumes that a pawn only changes a file by 1 when capturing.
    ///
    /// # Panics
    ///
    /// Will panic if there is no piece at the start square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let ply1 = Ply(Square::new("e2"), Square::new("e4"));
    /// let ply2 = Ply(Square::new("e2"), Square::new("d3"));
    /// assert!(!board.is_illegal_pawn_move(ply1));
    /// assert!(board.is_illegal_pawn_move(ply2));
    /// ```
    fn is_illegal_pawn_move(&self, ply: Ply) -> Result<Ply, &'static str> {
        let start_piece = self.get_piece(ply.start).unwrap();
        if !matches!(start_piece, Kind::Pawn(_c)) {
            return Ok(ply);
        }

        if ply.start.file == ply.dest.file {
            if self.get_piece(ply.dest).is_some() {
                return Err("Move is not valid. The pawn is capturing forward.");
            }
        } else if !ply.en_passant && self.get_piece(ply.dest).is_none() {
            return Err("Move is not valid. The pawn is moving diagonally without capturing.");
        }

        let ep_captured_piece = match start_piece {
            Kind::Pawn(Color::White) if ply.start.rank == 4 => self.get_piece(Square {
                rank: ply.dest.rank - 1,
                file: ply.dest.file,
            }),
            Kind::Pawn(Color::Black) if ply.start.rank == 3 => self.get_piece(Square {
                rank: ply.dest.rank + 1,
                file: ply.dest.file,
            }),
            _ => None,
        };

        if ply.en_passant
            && !(self
                .en_passant_file
                .is_some_and(|ep_file| ep_file == ply.dest.file)
                && ply.start.file != ply.dest.file
                && matches!(
                    ep_captured_piece,
                    Some(Kind::Pawn(color))
                    if color != start_piece.get_color()
                ))
        {
            return Err(
                "Move is not valid. The pawn is performing an en passant when it is not allowed.",
            );
        }

        Ok(ply)
    }

    /// Returns a boolean representing whether or not a given move is an illegal castling move
    ///
    /// Checks if castling rights are still availiable, and then ensures there
    /// are no pieces between the king and the rook and that the king never
    /// travels through check.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let ply = Ply(Square::new("e1"), Square::new("g1"));
    /// assert!(board.is_illegal_castling(ply));
    ///
    fn is_illegal_castling(&mut self, ply: Ply) -> Result<Ply, &'static str> {
        if !ply.is_castles {
            return Ok(ply);
        }

        match (self.current_turn, &ply.dest) {
            (Color::White, Square { rank: 0, file: 6 }) => (self.kingside_castle_status()
                == CastlingStatus::Availiable
                && self
                    .no_pieces_between_castling(CastlingKind::WhiteKingside)
                    .and(self.no_checks_between(Square::from("e1"), Square::from("g1")))
                    .is_ok())
            .then_some(ply)
            .ok_or("Move is not valid. The white king cannot castle kingside."),

            (Color::White, Square { rank: 0, file: 2 }) => (self.queenside_castle_status()
                == CastlingStatus::Availiable
                && self
                    .no_pieces_between_castling(CastlingKind::WhiteQueenside)
                    .and(self.no_checks_between(Square::from("e1"), Square::from("c1")))
                    .is_ok())
            .then_some(ply)
            .ok_or("Move is not valid. The white king cannot castle queenside."),

            (Color::Black, Square { rank: 7, file: 6 }) => (self.kingside_castle_status()
                == CastlingStatus::Availiable
                && self
                    .no_pieces_between_castling(CastlingKind::BlackKingside)
                    .and(self.no_checks_between(Square::from("e1"), Square::from("g1")))
                    .is_ok())
            .then_some(ply)
            .ok_or("Move is not valid. The black king cannot castle kingside."),

            (Color::Black, Square { rank: 7, file: 2 }) => (self.queenside_castle_status()
                == CastlingStatus::Availiable
                && self
                    .no_pieces_between_castling(CastlingKind::BlackQueenside)
                    .and(self.no_checks_between(Square::from("e1"), Square::from("g1")))
                    .is_ok())
            .then_some(ply)
            .ok_or("Move is not valid. The black king cannot castle queenside."),

            _ => Err(
                "Move is not valid. The destination square is not a valid castling destination.",
            ),
        }
    }

    /// Skip the current turn if possible, updating the state information of the board
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// board.skip_turn();
    /// assert_eq!(Color::Black, board.current_turn);
    /// ```
    #[allow(dead_code)]
    pub fn skip_turn(&mut self) {
        self.switch_turn();
    }

    /// Reverses a skiped turn, updating the state information of the board
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// board.skip_turn();
    /// assert_eq!(Color::Black, board.current_turn);
    /// board.undo_skip_turn();
    /// assert_eq!(Color::White, board.current_turn);
    /// ```
    #[allow(dead_code)]
    pub fn undo_skip_turn(&mut self) {
        self.switch_turn();
    }

    /// Switches the current turn to the other player
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// board.switch_turn();
    /// assert_eq!(Color::Black, board.current_turn);
    /// board.switch_turn();
    /// assert_eq!(Color::White, board.current_turn);
    /// ```
    pub fn switch_turn(&mut self) {
        self.current_turn = self.current_turn.opposite();
    }

    /// Returns a Result representing whether or not there are no pieces between two squares
    ///
    /// # Arguments
    ///
    /// * `start` - The starting square
    /// * `dest` - The destination square
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert!(board.no_pieces_between(Square::new("a4"), Square::new("h4")).is_ok());
    /// assert!(board.no_pieces_between(Square::new("a1"), Square::new("h1")).is_err());
    /// ```
    fn no_pieces_between(&self, start: Square, dest: Square) -> Result<(), &'static str> {
        let squares = start.get_transit_squares(dest);
        if squares.into_iter().any(|sq| self.get_piece(sq).is_some()) {
            Err("There are pieces between the start and destination squares.")
        } else {
            Ok(())
        }
    }

    fn no_pieces_between_castling(&self, kind: CastlingKind) -> Result<(), &'static str> {
        let pieces_blocking = match kind {
            CastlingKind::WhiteKingside => self.bitboards.all_pieces & 0xE,
            CastlingKind::WhiteQueenside => self.bitboards.all_pieces & 0x60,
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
    /// let board = Board::construct_starting_board();
    /// assert!(board.no_checks_between(Square::new("a1"), Square::new("h1")).is_ok());
    /// assert!(board.no_checks_between(Square::new("a8"), Square::new("h8")).is_err());
    /// ```
    fn no_checks_between(&mut self, start: Square, dest: Square) -> Result<(), &'static str> {
        let squares = start.get_transit_squares(dest);
        if squares
            .into_iter()
            .all(|sq| self.is_legal_move(Ply::new(start, sq)).is_ok())
        {
            Ok(())
        } else {
            Err("There are checks between the start and destination squares.")
        }
    }

    /// Returns a boolean representing whether or not the current side is in check
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert!(!board.is_in_check());
    /// ```
    #[allow(dead_code)]
    pub fn is_in_check(&self) -> bool {
        let attacking_pieces = match self.current_turn {
            Color::White => self.bitboards.black_pieces,
            Color::Black => self.bitboards.white_pieces,
        };

        let mut attacks = Bitboard::new(0);
        for square in 0..64u8 {
            if attacking_pieces & 1 << square == Bitboard::new(0) {
                continue;
            }

            let piece = self
                .get_piece(Square::from(square))
                .expect("No piece found at {square} where bitboard claimed piece was!");

            attacks |= piece.get_attacks(Square::from(square), self);
        }

        let king_pos = match self.current_turn {
            Color::White => self.bitboards.white_king,
            Color::Black => self.bitboards.black_king,
        };

        king_pos & attacks != Bitboard::new(0)
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
    /// let board = Board::construct_starting_board();
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
    /// let board = Board::construct_starting_board();
    /// // Playing with rook odds
    /// board.remove_piece(&Square::new("a1"), &PieceKind::Rook(Color::White));
    /// ```
    pub fn remove_piece(&mut self, square: Square, piece: Kind) {
        self.bitboards.remove_piece(square, piece);
    }

    /// Makes a half-move on this board
    ///
    /// # Arguments
    ///
    /// * `new_move` - A Ply that holds the origin and destination square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// // Move the a pawn one square forward
    /// board.make_move(Ply::new(Square::new("a2"), Square::new("a3")));
    /// ```
    pub fn make_move(&mut self, new_move: Ply) {
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

        if let (Some(promoted_to), Some(Kind::Pawn(c))) =
            (new_move.promoted_to, self.get_piece(new_move.dest))
        {
            self.remove_piece(new_move.dest, Kind::Pawn(c));
            self.add_piece(new_move.dest, promoted_to);
        }

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
                Square { rank: 0, file: 6 } => {
                    self.white_kingside_castling = CastlingStatus::Unavailiable;
                }
                Square { rank: 0, file: 2 } => {
                    self.white_queenside_castling = CastlingStatus::Unavailiable;
                }
                Square { rank: 7, file: 6 } => {
                    self.black_kingside_castling = CastlingStatus::Unavailiable;
                }
                Square { rank: 7, file: 2 } => {
                    self.black_queenside_castling = CastlingStatus::Unavailiable;
                }
                _ => panic!("Invalid castling king destination {}", new_move.dest),
            };
        }

        self.switch_turn();
        self.history.push(new_move);
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
    /// let board = Board::construct_starting_board();
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

            match old_move.dest {
                Square { rank: 0, file: 6 } => {
                    self.white_kingside_castling = CastlingStatus::Availiable;
                }
                Square { rank: 0, file: 2 } => {
                    self.white_queenside_castling = CastlingStatus::Availiable;
                }
                Square { rank: 7, file: 6 } => {
                    self.black_kingside_castling = CastlingStatus::Availiable;
                }
                Square { rank: 7, file: 2 } => {
                    self.black_queenside_castling = CastlingStatus::Availiable;
                }
                _ => panic!("Invalid castling king destination {}", old_move.dest),
            };
        }

        if self
            .history
            .last()
            .map_or(false, |mv| mv.is_double_pawn_push)
        {
            self.en_passant_file = Some(self.history.last().unwrap().start.file);
        } else {
            self.en_passant_file = None;
        }

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
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_piece1() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::from("a1")).unwrap(),
            Kind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::from("h8")).unwrap(),
            Kind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::from("h7")).unwrap(),
            Kind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = Board::construct_starting_board();
        assert!(board.get_piece(Square::from("e5")).is_none());
    }

    #[test]
    #[should_panic = "attempt to shift left with overflow"]
    fn test_get_piece_ooblack_rank() {
        let board = Board::construct_starting_board();
        board.get_piece(Square { rank: 8, file: 7 }).unwrap();
    }

    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value"]
    fn test_get_piece_ooblack_file() {
        let board = Board::construct_starting_board();
        board.get_piece(Square { rank: 0, file: 8 }).unwrap();
    }

    #[test]
    fn test_get_all_moves() {
        let board = Board::construct_starting_board();
        let all_moves = board.get_all_moves();

        assert!(!all_moves.is_empty());
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::construct_starting_board();
        let square = Square::from("a3");
        board.add_piece(square, Kind::Queen(Color::White));
        assert_eq!(board.get_piece(square).unwrap(), Kind::Queen(Color::White));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::construct_starting_board();
        let square = Square::from("a2");

        // Should do nothing, since there is a white pawn here, not a black pawn
        board.remove_piece(square, Kind::Pawn(Color::Black));
        assert_eq!(board.get_piece(square).unwrap(), Kind::Pawn(Color::White));

        board.remove_piece(square, Kind::Pawn(Color::White));
        assert!(board.get_piece(square).is_none());
    }

    #[test]
    fn test_board_display() {
        let board = Board::construct_starting_board();
        let correct =
            "♖♘♗♕♔♗♘♖\n♙♙♙♙♙♙♙♙\n--------\n--------\n--------\n--------\n♟♟♟♟♟♟♟♟\n♜♞♝♛♚♝♞♜\n";
        assert_eq!(board.to_string(), correct);
    }

    #[test]
    fn test_is_white_turn() {
        let board = Board::construct_starting_board();
        assert!(board.current_turn == Color::White);
    }

    #[test]
    fn test_is_black_turn() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        assert!(board.current_turn == Color::Black);
    }

    #[test]
    fn test_kingside_castle_true() {
        let mut board = Board::construct_starting_board();
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Availiable);
        board.skip_turn();
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Availiable);
    }

    #[test]
    fn test_queenside_castle_true() {
        let mut board = Board::construct_starting_board();
        assert_eq!(board.queenside_castle_status(), CastlingStatus::Availiable);
        board.skip_turn();
        assert_eq!(board.queenside_castle_status(), CastlingStatus::Availiable);
    }

    #[test]
    fn test_kingside_castle_false_white() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qkq - 0 1");
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Unavailiable);
        board.skip_turn();
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Availiable);
    }

    #[test]
    fn test_kingside_castle_false_black() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQq - 0 1");
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Availiable);
        board.skip_turn();
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Unavailiable);
    }

    #[test]
    fn test_kingside_castle_false_both() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 1");
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Unavailiable);
        board.skip_turn();
        assert_eq!(board.kingside_castle_status(), CastlingStatus::Unavailiable);
    }

    #[test]
    fn test_queenside_castle_false_white() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kkq - 0 1");
        assert_eq!(
            board.queenside_castle_status(),
            CastlingStatus::Unavailiable
        );
        board.skip_turn();
        assert_eq!(board.queenside_castle_status(), CastlingStatus::Availiable);
    }

    #[test]
    fn test_queenside_castle_false_black() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk - 0 1");
        assert_eq!(board.queenside_castle_status(), CastlingStatus::Availiable);
        board.skip_turn();
        assert_eq!(
            board.queenside_castle_status(),
            CastlingStatus::Unavailiable
        );
    }

    #[test]
    fn test_queenside_castle_false_both() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kk - 0 1");
        assert_eq!(
            board.queenside_castle_status(),
            CastlingStatus::Unavailiable
        );
        board.skip_turn();
        assert_eq!(
            board.queenside_castle_status(),
            CastlingStatus::Unavailiable
        );
    }

    #[test]
    fn test_make_unmake_move_single() {
        let mut board = Board::construct_starting_board();
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
        let mut board = Board::construct_starting_board();
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
        let mut board = Board::construct_starting_board();
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
        let board = Board::construct_starting_board();
        assert!(!board.is_in_check());
    }

    #[test]
    fn test_is_in_check_white_by_queen() {
        let board = Board::from_fen("8/1k6/2q5/8/8/2K3Q1/8/8 w - - 0 1");
        assert!(board.is_in_check());
    }

    #[test]
    fn test_is_in_check_black_by_queen() {
        let board = Board::from_fen("8/1K6/2Q5/8/8/2k3q1/8/8 b - - 0 1");
        assert!(board.is_in_check());
    }

    #[test]
    fn test_get_legal_moves_count_start() {
        let mut board = Board::construct_starting_board();
        let result = board.get_legal_moves().len();
        let correct = 20;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_1() {
        let mut board = Board::from_fen("2k1b3/8/8/8/2K5/5Q2/5PPP/5RN1 w - - 0 1");
        println!("{board}");
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
        dbg!(board.get_legal_moves());
        let result = board.get_legal_moves().len();
        let correct = 26;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_10() {
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        println!("{:}", board);
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
}
