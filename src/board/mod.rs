use crate::utils::FENInstruction;
use std::collections::HashMap;
use std::fmt;

mod boardbuilder;
pub mod piece;
mod ply;
mod square;

use boardbuilder::BoardBuilder;
use piece::{Color, Kind};
use ply::Ply;
use square::Square;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Castling {
    #[default]
    Availiable,
    Unavailiable,
}

// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    is_white_turn: bool,

    w_kingside_castling: Castling,
    w_queenside_castling: Castling,
    b_kingside_castling: Castling,
    b_queenside_castling: Castling,

    w_pawns: u64,
    w_king: u64,
    w_queens: u64,
    w_rooks: u64,
    w_bishops: u64,
    w_knights: u64,
    b_pawns: u64,
    b_king: u64,
    b_queens: u64,
    b_rooks: u64,
    b_bishops: u64,
    b_knights: u64,

    history: Vec<Ply>,
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

    /// Returns a new board given a FEN string
    ///
    /// # Examples
    /// ```
    /// let board = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");
    /// ```
    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Self {
        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();

        let mut w_pawns: u64 = 0;
        let mut w_king: u64 = 0;
        let mut w_queens: u64 = 0;
        let mut w_rooks: u64 = 0;
        let mut w_bishops: u64 = 0;
        let mut w_knights: u64 = 0;
        let mut b_pawns: u64 = 0;
        let mut b_king: u64 = 0;
        let mut b_queens: u64 = 0;
        let mut b_rooks: u64 = 0;
        let mut b_bishops: u64 = 0;
        let mut b_knights: u64 = 0;

        let mut idx: u64 = 0;
        for chr in fields[0].chars() {
            let instruction = match chr {
                'P' => FENInstruction::Bitboard(&mut w_pawns),
                'K' => FENInstruction::Bitboard(&mut w_king),
                'Q' => FENInstruction::Bitboard(&mut w_queens),
                'R' => FENInstruction::Bitboard(&mut w_rooks),
                'B' => FENInstruction::Bitboard(&mut w_bishops),
                'N' => FENInstruction::Bitboard(&mut w_knights),
                'p' => FENInstruction::Bitboard(&mut b_pawns),
                'k' => FENInstruction::Bitboard(&mut b_king),
                'q' => FENInstruction::Bitboard(&mut b_queens),
                'r' => FENInstruction::Bitboard(&mut b_rooks),
                'b' => FENInstruction::Bitboard(&mut b_bishops),
                'n' => FENInstruction::Bitboard(&mut b_knights),
                '1'..='8' => FENInstruction::Skip(chr.to_string().parse().ok().unwrap()),
                '/' => FENInstruction::NewRow(),
                _ => panic!("Unknown FEN instruction: {chr}"),
            };

            let mask: u64 = 1 << (63 - idx);
            match instruction {
                FENInstruction::Bitboard(bb) => *bb |= mask,
                FENInstruction::Skip(num) => idx += num - 1,
                FENInstruction::NewRow() => idx -= 1,
            }
            idx += 1;
        }

        let is_white_turn = match fields[1].chars().next().unwrap_or('w') {
            'w' => true,
            'b' => false,
            _ => panic!("Not given a valid FEN. The second field must either be a 'b' or a 'w'"),
        };

        let mut w_kingside_castling: Castling = Castling::Unavailiable;
        let mut b_kingside_castling: Castling = Castling::Unavailiable;
        let mut w_queenside_castling: Castling = Castling::Unavailiable;
        let mut b_queenside_castling: Castling = Castling::Unavailiable;

        for chr in fields[2].chars() {
            match chr {
                'K' => w_kingside_castling = Castling::Availiable,
                'k' => b_kingside_castling = Castling::Availiable,
                'Q' => w_queenside_castling = Castling::Availiable,
                'q' => b_queenside_castling = Castling::Availiable,
                '-' => (),
                _ => panic!("Unknown FEN castling notation: {chr}"),
            };
        }

        // TODO: En passant target square
        // TODO: Halfmove clock
        // TODO: Fullmove number

        Self {
            is_white_turn,

            w_kingside_castling,
            w_queenside_castling,
            b_kingside_castling,
            b_queenside_castling,

            history: Vec::new(),
            w_pawns,
            w_king,
            w_queens,
            w_rooks,
            w_bishops,
            w_knights,
            b_pawns,
            b_king,
            b_queens,
            b_rooks,
            b_bishops,
            b_knights,
        }
    }

    /// Creates a new board object that represents the starting board state in a normal game
    pub const fn construct_starting_board() -> Self {
        Self {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

            w_pawns: 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            w_king: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            w_queens: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            w_rooks: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            w_bishops: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            w_knights: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            b_pawns: 0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            b_king: 0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            b_queens: 0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            b_rooks: 0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            b_bishops: 0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            b_knights: 0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,

            history: Vec::new(),
        }
    }

    /// Returns a boolean representing whether or not it is white's turn
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert!(board.is_white_turn());
    /// ```
    #[allow(dead_code)]
    pub const fn is_white_turn(&self) -> bool {
        self.is_white_turn
    }

    /// Returns a boolean representing whether or not the specified player has kingside castling rights
    ///
    /// # Arguments
    ///
    /// * `white` - A boolean representing whether or not we are checking white or black's castling rights
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert!(board.has_kingside_castle(true));
    /// assert!(board.has_kingside_castle(false));
    /// ```
    #[allow(dead_code)]
    pub const fn kingside_castle_status(&self, color: Color) -> Castling {
        match color {
            Color::White => self.w_kingside_castling,
            Color::Black => self.b_kingside_castling,
        }
    }

    /// Returns a boolean representing whether or not the specified player has queenside castling rights
    ///
    /// # Arguments
    ///
    /// * `white` - A boolean representing whether or not we are checqueen white or black's castling rights
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// assert!(board.has_queenside_castle(true));
    /// assert!(board.has_queenside_castle(false));
    /// ```
    #[allow(dead_code)]
    pub const fn queenside_castle_status(&self, color: Color) -> Castling {
        match color {
            Color::White => self.w_queenside_castling,
            Color::Black => self.b_queenside_castling,
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
        let mask = square.get_mask();
        for (kind, bb) in self.bitboard_map() {
            if (*bb & mask) >= 1 {
                return Some(kind);
            }
        }
        None
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
        for i in (0..8).rev() {
            for j in 0..8 {
                let square = Square { rank: i, file: j };
                if let Some(piece) = self.get_piece(square) {
                    if !self.is_white_turn ^ (piece.get_color() == Color::White) {
                        let mut square_moveset = piece
                            .get_moveset(square)
                            .into_iter()
                            .map(|mut mv| {
                                mv.captured_piece = self.get_piece(mv.dest);
                                mv
                            })
                            .collect::<Vec<Ply>>();
                        all_moves.append(&mut square_moveset);
                    }
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
    pub fn get_legal_moves(&self) -> Vec<Ply> {
        self.filter_moves(self.get_all_moves(), 1)
    }

    /// Filters a list of moves to only include legal moves
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board()
    /// let movelist = board.get_all_moves(Square::new("e1"));
    /// let legal_moves = filter_moves(&board, movelist);
    /// ```
    fn filter_moves(&self, moves: Vec<Ply>, depth: u64) -> Vec<Ply> {
        moves
            .into_iter()
            .filter(|mv| self.is_legal_move(*mv, depth).is_ok())
            .collect()
    }

    /// Returns a boolean representing whether or not a given move is legal
    ///
    /// # Examples
    /// ```
    /// let ply = Ply(Square::new("e2"), Square::new("e9"));
    /// assert!(!board.is_legal_move(ply));
    /// ```
    fn is_legal_move(&self, ply: Ply, depth: u64) -> Result<Ply, &'static str> {
        let result: Result<Ply, &'static str> = self
            .is_on_board(ply)
            .and_then(|ply| self.is_self_capture(ply))
            .and_then(|ply| self.is_illegal_jump(ply))
            .and_then(|ply| self.is_illegal_pawn_move(ply))
            .and_then(|ply| self.is_illegal_castling(ply));

        if result.is_err() || depth == 0 {
            return result;
        }
        // Don't allow leaving your king in check
        if self.is_in_check_helper(depth - 1, Some(ply)) {
            Err("Move is not valid. The move would leave the king in check.")
        } else {
            Ok(ply)
        }
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
            _ => self
                .no_pieces_between(ply.start, ply.dest)
                .then_some(ply)
                .ok_or("Move is not valid. The move jumps through other pieces."),
        }
    }

    /// Returns a boolean representing whether or not a given move is an illegal pawn move
    ///
    /// Checks if a pawn is capturing forward, or moving diagonally without capturing.
    ///
    /// # Panics
    ///
    /// Will panic if there is no piece at the start square of the move.
    ///   return false;

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
        } else if self.get_piece(ply.dest).is_none() {
            return Err("Move is not valid. The pawn is moving diagonally without capturing.");
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
    fn is_illegal_castling(&self, ply: Ply) -> Result<Ply, &'static str> {
        if !ply.is_castles {
            return Ok(ply);
        }
        match &ply.dest {
            Square { rank: 0, file: 6 } => (self.kingside_castle_status(Color::White)
                == Castling::Availiable
                && self.no_pieces_between(Square::new("e1"), Square::new("h1"))
                && self.no_checks_between(Square::new("e1"), Square::new("g1")))
            .then_some(ply)
            .ok_or("Move is not valid. The white king cannot castle kingside."),
            Square { rank: 0, file: 2 } => (self.queenside_castle_status(Color::White)
                == Castling::Availiable
                && self.no_pieces_between(Square::new("e1"), Square::new("a1"))
                && self.no_checks_between(Square::new("e1"), Square::new("c1")))
            .then_some(ply)
            .ok_or("Move is not valid. The white king cannot castle queenside."),
            Square { rank: 7, file: 6 } => (self.kingside_castle_status(Color::Black)
                == Castling::Availiable
                && self.no_pieces_between(Square::new("e8"), Square::new("g8"))
                && self.no_checks_between(Square::new("e1"), Square::new("g1")))
            .then_some(ply)
            .ok_or("Move is not valid. The black king cannot castle kingside."),
            Square { rank: 7, file: 2 } => (self.queenside_castle_status(Color::Black)
                == Castling::Availiable
                && self.no_pieces_between(Square::new("e8"), Square::new("c8"))
                && self.no_checks_between(Square::new("e1"), Square::new("g1")))
            .then_some(ply)
            .ok_or("Move is not valid. The black king cannot castle queenside."),
            _ => Err(
                "Move is not valid. The destination square is not a valid castling destination.",
            ),
        }
    }

    pub fn skip_turn(&mut self) {
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn undo_skip_turn(&mut self) {
        self.skip_turn();
    }

    fn no_pieces_between(&self, start: Square, dest: Square) -> bool {
        let squares = start.get_transit_squares(dest);
        squares.into_iter().all(|sq| self.get_piece(sq).is_none())
    }

    fn no_checks_between(&self, start: Square, dest: Square) -> bool {
        let squares = start.get_transit_squares(dest);
        squares
            .into_iter()
            .all(|sq| self.is_legal_move(Ply::new(start, sq), 1).is_ok())
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
        self.is_in_check_helper(1, None)
    }

    pub fn is_in_check_helper(&self, depth: u64, ply: Option<Ply>) -> bool {
        let mut board_copy = self.clone();
        if let Some(ply) = ply {
            board_copy.make_move(ply);
        } else {
            board_copy.skip_turn();
        }
        let enemy_moves = board_copy.filter_moves(board_copy.get_all_moves(), depth);
        let result = enemy_moves.into_iter().any( |mv| mv.captured_piece.is_some_and( |pc| matches!(pc, Kind::King(c) if c != board_copy.get_piece(mv.start).unwrap().get_color())));
        board_copy.undo_skip_turn();

        result
    }

    /// Returns a `HashMap` of `PieceKinds` to a reference of their corresponding bitboard
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let all_bb = board.bitboard_map();
    /// let pawn_bb: u64 = all_bb.get(PieceKind::Pawn(Color::White));
    /// ```
    fn bitboard_map(&self) -> HashMap<Kind, &u64> {
        let mut output: HashMap<Kind, &u64> = HashMap::new();

        output.insert(Kind::Pawn(Color::White), &self.w_pawns);
        output.insert(Kind::King(Color::White), &self.w_king);
        output.insert(Kind::Queen(Color::White), &self.w_queens);
        output.insert(Kind::Rook(Color::White), &self.w_rooks);
        output.insert(Kind::Bishop(Color::White), &self.w_bishops);
        output.insert(Kind::Knight(Color::White), &self.w_knights);

        output.insert(Kind::Pawn(Color::Black), &self.b_pawns);
        output.insert(Kind::King(Color::Black), &self.b_king);
        output.insert(Kind::Queen(Color::Black), &self.b_queens);
        output.insert(Kind::Rook(Color::Black), &self.b_rooks);
        output.insert(Kind::Bishop(Color::Black), &self.b_bishops);
        output.insert(Kind::Knight(Color::Black), &self.b_knights);

        output
    }

    /// Returns a `HashMap` of `PieceKinds` to a reference of their corresponding, mutable bitboard
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// let all_bb = board.bitboard_map_mut();
    /// all_bb[PieceKind::Pawn(Color::White)] |= 0x1;
    /// ```
    fn bitboard_map_mut(&mut self) -> HashMap<Kind, &mut u64> {
        let mut output: HashMap<Kind, &mut u64> = HashMap::new();

        output.insert(Kind::Pawn(Color::White), &mut self.w_pawns);
        output.insert(Kind::King(Color::White), &mut self.w_king);
        output.insert(Kind::Queen(Color::White), &mut self.w_queens);
        output.insert(Kind::Rook(Color::White), &mut self.w_rooks);
        output.insert(Kind::Bishop(Color::White), &mut self.w_bishops);
        output.insert(Kind::Knight(Color::White), &mut self.w_knights);

        output.insert(Kind::Pawn(Color::Black), &mut self.b_pawns);
        output.insert(Kind::King(Color::Black), &mut self.b_king);
        output.insert(Kind::Queen(Color::Black), &mut self.b_queens);
        output.insert(Kind::Rook(Color::Black), &mut self.b_rooks);
        output.insert(Kind::Bishop(Color::Black), &mut self.b_bishops);
        output.insert(Kind::Knight(Color::Black), &mut self.b_knights);

        output
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
        let mask = square.get_mask();
        self.bitboard_map_mut()
            .entry(piece)
            .and_modify(|bb| **bb |= mask);
    }

    /// Removes any piece from the specified square
    ///
    /// # Arguments
    ///
    /// * `square` - A square on the board to clear
    ///
    /// # Examples
    /// ```
    /// let board = Board::construct_starting_board();
    /// // Playing with rook odds
    /// board.clear_piece(&Square::new("a1"));
    /// ```
    #[allow(dead_code)]
    pub fn clear_piece(&mut self, square: Square) {
        let mask = !square.get_mask();
        self.bitboard_map_mut()
            .iter_mut()
            .for_each(|(_, bb)| **bb &= mask);
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
        let mask = !square.get_mask();
        self.bitboard_map_mut()
            .entry(piece)
            .and_modify(|bb| **bb &= mask);
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
        let dest_piece_kind = self.replace_square(new_move.start, new_move.dest);
        assert_eq!(new_move.captured_piece, dest_piece_kind);

        if new_move.is_castles {
            let (rook_start, rook_dest) = match new_move.dest {
                Square { rank: 0, file: 6 } => (Square::new("h1"), Square::new("f1")),
                Square { rank: 0, file: 2 } => (Square::new("a1"), Square::new("d1")),
                Square { rank: 7, file: 6 } => (Square::new("h8"), Square::new("f8")),
                Square { rank: 7, file: 2 } => (Square::new("a8"), Square::new("d8")),
                _ => panic!("Invalid castling king destination {}", new_move.dest),
            };

            self.replace_square(rook_start, rook_dest);

            match new_move.dest {
                Square { rank: 0, file: 6 } => self.w_kingside_castling = Castling::Unavailiable,
                Square { rank: 0, file: 2 } => self.w_queenside_castling = Castling::Unavailiable,
                Square { rank: 7, file: 6 } => self.b_kingside_castling = Castling::Unavailiable,
                Square { rank: 7, file: 2 } => self.b_queenside_castling = Castling::Unavailiable,
                _ => panic!("Invalid castling king destination {}", new_move.dest),
            };
        }

        self.is_white_turn = !self.is_white_turn;
        self.history.push(new_move);
    }

    fn replace_square(&mut self, start: Square, dest: Square) -> Option<Kind> {
        let start_piece_kind = self.get_piece(start).unwrap();
        self.remove_piece(start, start_piece_kind);

        let dest_piece_kind_option = self.get_piece(dest);
        if let Some(dest_piece_kind) = dest_piece_kind_option {
            self.remove_piece(dest, dest_piece_kind);
        }

        self.add_piece(dest, start_piece_kind);

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
    pub fn unmake_move(&mut self, old_move: Ply) {
        self.replace_square(old_move.dest, old_move.start);

        if let Some(captured_piece) = self.history.pop().unwrap().captured_piece {
            self.add_piece(old_move.dest, captured_piece);
        }

        if old_move.is_castles {
            let (rook_start, rook_dest) = match old_move.dest {
                Square { rank: 0, file: 6 } => (Square::new("h1"), Square::new("f1")),
                Square { rank: 0, file: 2 } => (Square::new("a1"), Square::new("d1")),
                Square { rank: 7, file: 6 } => (Square::new("h8"), Square::new("f8")),
                Square { rank: 7, file: 2 } => (Square::new("a8"), Square::new("d8")),
                _ => panic!("Invalid castling king destination {}", old_move.dest),
            };

            self.replace_square(rook_dest, rook_start);

            match old_move.dest {
                Square { rank: 0, file: 6 } => self.w_kingside_castling = Castling::Availiable,
                Square { rank: 0, file: 2 } => self.w_queenside_castling = Castling::Availiable,
                Square { rank: 7, file: 6 } => self.b_kingside_castling = Castling::Availiable,
                Square { rank: 7, file: 2 } => self.b_queenside_castling = Castling::Availiable,
                _ => panic!("Invalid castling king destination {}", old_move.dest),
            };
        }

        self.is_white_turn = !self.is_white_turn;
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

    #[test]
    fn from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Board::construct_starting_board(), Board::from_fen(fen));
    }

    #[test]
    fn from_fen_white_position1() {
        let fen = "1k1r3r/p6p/1pp1pp2/2Np1qp1/1Q1P4/2P1PP2/PP4PP/R4nK1 w - - 0 21";
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Unavailiable,
            w_queenside_castling: Castling::Unavailiable,
            b_kingside_castling: Castling::Unavailiable,
            b_queenside_castling: Castling::Unavailiable,

            w_pawns: 271_368_960,
            w_king: 2,
            w_queens: 1_073_741_824,
            w_rooks: 128,
            w_bishops: 0,
            w_knights: 137_438_953_472,
            b_pawns: 36_429_096_560_885_760,
            b_king: 4_611_686_018_427_387_904,
            b_queens: 17_179_869_184,
            b_rooks: 1_224_979_098_644_774_912,
            b_bishops: 0,
            b_knights: 4,
            history: Vec::new(),
        };

        assert_eq!(Board::from_fen(fen), correct);
    }

    #[test]
    fn from_fen_black_position1() {
        let fen = "5b2/pp1N2pk/2pn1q1p/3n1p1Q/3P1P2/2PB3R/PP3KPP/R1B1r3 b - - 12 31";
        let correct = Board {
            is_white_turn: false,

            w_kingside_castling: Castling::Unavailiable,
            w_queenside_castling: Castling::Unavailiable,
            b_kingside_castling: Castling::Unavailiable,
            b_queenside_castling: Castling::Unavailiable,

            w_pawns: 337_691_392,
            w_king: 1024,
            w_queens: 4_294_967_296,
            w_rooks: 65664,
            w_bishops: 1_048_608,
            w_knights: 4_503_599_627_370_496,
            b_pawns: 54_642_446_545_453_056,
            b_king: 281_474_976_710_656,
            b_queens: 4_398_046_511_104,
            b_rooks: 8,
            b_bishops: 288_230_376_151_711_744,
            b_knights: 17_660_905_521_152,
            history: Vec::new(),
        };

        assert_eq!(Board::from_fen(fen), correct);
    }

    #[test]
    fn test_get_piece1() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::new("a1")).unwrap(),
            Kind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::new("h8")).unwrap(),
            Kind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.get_piece(Square::new("h7")).unwrap(),
            Kind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = Board::construct_starting_board();
        assert!(board.get_piece(Square::new("e5")).is_none());
    }

    #[test]
    #[should_panic = "attempt to shift left with overflow"]
    fn test_get_piece_oob_rank() {
        let board = Board::construct_starting_board();
        board.get_piece(Square { rank: 8, file: 7 }).unwrap();
    }

    #[test]
    #[should_panic = "attempt to subtract with overflow"]
    fn test_get_piece_oob_file() {
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
        let square = Square::new("a3");
        board.add_piece(square, Kind::Queen(Color::White));
        assert_eq!(board.get_piece(square).unwrap(), Kind::Queen(Color::White));
    }

    #[test]
    fn test_clear_piece() {
        let mut board = Board::construct_starting_board();
        let square = Square::new("a2");
        board.clear_piece(square);
        assert!(board.get_piece(square).is_none());
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::construct_starting_board();
        let square = Square::new("a2");

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
        assert!(board.is_white_turn());
    }

    #[test]
    fn test_is_black_turn() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        assert!(!board.is_white_turn());
    }

    #[test]
    fn test_kingside_castle_true() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
    }

    #[test]
    fn test_queenside_castle_true() {
        let board = Board::construct_starting_board();
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
    }

    #[test]
    fn test_kingside_castle_false() {
        let mut board = Board::construct_starting_board();
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );

        board.w_kingside_castling = Castling::Unavailiable;
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Unavailiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::Black),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );

        board.b_kingside_castling = Castling::Unavailiable;
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Unavailiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::Black),
            Castling::Unavailiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );
    }

    #[test]
    fn test_queenside_castle_false() {
        let mut board = Board::construct_starting_board();
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::Black),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );

        board.w_queenside_castling = Castling::Unavailiable;
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Unavailiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::Black),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );

        board.b_queenside_castling = Castling::Unavailiable;
        assert_eq!(
            board.queenside_castle_status(Color::White),
            Castling::Unavailiable
        );
        assert_eq!(
            board.queenside_castle_status(Color::Black),
            Castling::Unavailiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
        assert_eq!(
            board.kingside_castle_status(Color::White),
            Castling::Availiable
        );
    }

    #[test]
    fn test_make_unmake_move_single() {
        let mut board = Board::construct_starting_board();
        let start = Square::new("a2");
        let dest = Square::new("a3");
        let ply = Ply::new(start, dest);

        assert!(board.is_white_turn);

        assert!(board.get_piece(dest).is_none());
        board.make_move(ply);
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::White));
        assert!(!board.is_white_turn);

        assert!(board.get_piece(start).is_none());

        board.unmake_move(ply);
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert!(board.is_white_turn);

        assert!(board.get_piece(dest).is_none());
    }

    #[test]
    fn test_make_unmake_move_double() {
        // Make and unmake two moves in a row
        let mut board = Board::construct_starting_board();
        let start = Square::new("a2");
        let dest1 = Square::new("a3");
        let dest2 = Square::new("a4");
        let ply1 = Ply::new(start, dest1);
        let ply2 = Ply::new(dest1, dest2);

        assert!(board.is_white_turn);

        assert!(board.get_piece(dest1).is_none());
        assert!(board.get_piece(dest2).is_none());
        board.make_move(ply1);
        assert_eq!(board.get_piece(dest1).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(start).is_none());
        assert!(board.get_piece(dest2).is_none());
        assert!(!board.is_white_turn);

        board.make_move(ply2);
        assert_eq!(board.get_piece(dest2).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(start).is_none());
        assert!(board.get_piece(dest1).is_none());
        assert!(board.is_white_turn);

        board.unmake_move(ply2);
        assert_eq!(board.get_piece(dest1).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(dest2).is_none());
        assert!(board.get_piece(start).is_none());
        assert!(!board.is_white_turn);

        board.unmake_move(ply1);
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(dest2).is_none());
        assert!(board.get_piece(dest1).is_none());
        assert!(board.is_white_turn);
    }

    #[test]
    fn test_make_unmake_move_capture() {
        let mut board = Board::construct_starting_board();
        let start = Square::new("a2"); // White Pawn
        let dest = Square::new("a7"); // Black Pawn
        let ply = Ply::builder(start, dest)
            .captured(Kind::Pawn(Color::Black))
            .build();
        assert!(board.is_white_turn);

        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::Black));
        board.make_move(ply);
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::White));
        assert!(board.get_piece(start).is_none());
        assert!(!board.is_white_turn);

        board.unmake_move(ply);
        assert_eq!(board.get_piece(start).unwrap(), Kind::Pawn(Color::White));
        assert_eq!(board.get_piece(dest).unwrap(), Kind::Pawn(Color::Black));
        assert!(board.is_white_turn);
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
        let board = Board::construct_starting_board();
        let result = board.get_legal_moves().len();
        let correct = 20;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_1() {
        let board = Board::from_fen("2k1b3/8/8/8/2K5/5Q2/5PPP/5RN1 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 39;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_2() {
        let board = Board::from_fen("8/1K6/2Q5/8/8/6q1/2k5/8 b - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_3() {
        let board = Board::from_fen("8/1K6/2Q5/8/8/6q1/2k5/8 b - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 7;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_4() {
        let board = Board::from_fen("8/1k6/2q5/5b2/8/R5Q1/2K5/3N4 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 3;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_5() {
        let board = Board::from_fen("8/1k6/2q5/8/8/R5Q1/2K5/3N4 w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 8;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_6() {
        let board = Board::from_fen("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 50;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_7() {
        let board = Board::from_fen("rnbqkbnr/8/5B2/Q2B4/3R2N1/2N5/1K6/7R w kq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 72;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_8() {
        let board = Board::from_fen("5b2/r7/1qn2B1n/1Q6/3R2N1/2N3k1/1K2Br2/3b3R w - - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 44;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_9() {
        let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 26;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_10() {
        let board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let result = board.get_legal_moves().len();
        let correct = 25;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_11() {
        let board = Board::from_fen("4r2k/4qpRb/2p1p2Q/1p3r1P/p2P4/P4P2/1PP1N3/1K4R1 b - - 2 32");
        let result = board.get_legal_moves().len();
        let correct = 31;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_12() {
        let board =
            Board::from_fen("r2qk2r/pp3ppb/2p1pn1p/4Q2P/2B5/3P2N1/PPP2PP1/R3K2R b KQkq - 0 14");
        let result = board.get_legal_moves().len();
        let correct = 37;

        assert_eq!(result, correct);
    }

    #[test]
    fn test_get_legal_moves_count_from_position_13() {
        let board =
            Board::from_fen("r2q1rk1/pp3ppb/2p1pn1p/4Q2P/2B5/3P2N1/PPP2PP1/2KR3R b - - 2 15");
        let result = board.get_legal_moves().len();
        let correct = 33;

        assert_eq!(result, correct);
    }
}
