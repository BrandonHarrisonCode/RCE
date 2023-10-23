use std::collections::HashMap;
use std::fmt;

pub mod piece;
mod ply;
mod square;

use super::utils::FENInstruction;
use piece::{Color, PieceKind};
use ply::Ply;
use square::Square;

// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    is_white_turn: bool,

    w_kingside_castling: bool,
    w_queenside_castling: bool,
    b_kingside_castling: bool,
    b_queenside_castling: bool,

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
    /// Returns a new board given starting bitboards
    ///
    /// # Arguments
    ///
    /// * `w_pawns` - A u64 bitboard representing white pawns
    ///
    /// * `w_king` - A u64 bitboard representing the white king
    ///
    /// * `w_queens` - A u64 bitboard representing white queens
    ///
    /// * `w_rooks` - A u64 bitboard representing white rooks
    ///
    /// * `w_bishops` - A u64 bitboard representing white bishops
    ///
    /// * `w_knights` - A u64 bitboard representing white knights
    ///
    /// * `b_pawns` - A u64 bitboard representing black pawns
    ///
    /// * `b_king` - A u64 bitboard representing the black king
    ///
    /// * `b_queens` - A u64 bitboard representing black queens
    ///
    /// * `b_rooks` - A u64 bitboard representing black rooks
    ///
    /// * `b_bishops` - A u64 bitboard representing black bishops
    ///
    /// * `b_knights` - A u64 bitboard representing black knights
    ///
    /// # Examples
    /// ```
    /// // Create empty board
    /// let board = Board::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
    ) -> Board {
        Board {
            is_white_turn: true,
            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,


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

            history: Vec::new(),
        }
    }

    /// Returns a new, empty board
    ///
    /// # Examples
    /// ```
    /// // Create empty board
    /// let board = Board::new_empty_board();
    /// ```
    #[allow(dead_code)]
    pub fn new_empty_board() -> Board {
        Board::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    /// Returns a new board given a FEN string
    ///
    /// # Arguments
    ///
    /// * `fen` - A string representing the FEN of a position
    ///
    /// # Examples
    /// ```
    /// // Create empty board
    /// let board = Board::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    /// ```
    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Board {
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
                _ => panic!("Unknown FEN instruction: {}", chr),
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

        let mut w_kingside_castling: bool = false;
        let mut b_kingside_castling: bool = false;
        let mut w_queenside_castling: bool = false;
        let mut b_queenside_castling: bool = false;

        for chr in fields[2].chars() {
            match chr {
                'K' => w_kingside_castling = true,
                'k' => b_kingside_castling = true,
                'Q' => w_queenside_castling = true,
                'q' => b_queenside_castling = true,
                '-' => (),
                _ => panic!("Unknown FEN castling notation: {}", chr),
            };
        };


        // TODO: Castling rights
        // TODO: En passant target square
        // TODO: Halfmove clock
        // TODO: Fullmove number

        Board {
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

    /// Returns a PieceKind Option of the piece currently occupying `square`
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
    /// let board = create_starting_board();
    /// assert_eq!(PieceKind::Rook(Color::White), board.get_piece(Square::new("a1")));
    /// assert_eq!(None, board.get_piece(Square::new("b3")));
    /// ```
    pub fn get_piece(&self, square: &Square) -> Option<PieceKind> {
        let mask = square.get_mask();
        for (kind, bb) in self.bitboard_map() {
            if (*bb & mask) >= 1 {
                return Some(kind);
            }
        }
        None
    }

    /// Returns an optional list of Plys for the piece at a given square
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
    /// let board = create_starting_board();
    /// let movelist = board.get_moves_for_piece(Square::new("a2"));
    /// ```
    #[allow(dead_code)]
    pub fn get_moves_for_piece(&self, square: &Square) -> Option<Vec<Ply>> {
        self.get_piece(square)
            .map(|x| x.get_all_legal_moves(square))
    }

    /// Returns a list of all potential moves for the current side
    ///
    /// # Examples
    /// ```
    /// let board = create_starting_board();
    /// let movelist = board.get_all_moves(Square::new("a2"));
    /// ```
    pub fn get_all_moves(&self) -> Vec<Ply> {
        let mut all_moves = Vec::new();
        for i in (0..8).rev() {
            for j in 0..8 {
                let square = &Square { rank: i, file: j };
                if let Some(piece) = self.get_piece(square) {
                    if !self.is_white_turn ^ (piece.get_color() == Color::White) {
                        all_moves.append(&mut piece.get_all_legal_moves(square));
                    }
                }
            }
        }
        all_moves
    }

    /// Returns a HashMap of PieceKinds to a reference of their corresponding bitboard
    ///
    /// # Examples
    /// ```
    /// let board = create_starting_board();
    /// let all_bb = board.bitboard_map();
    /// let pawn_bb: u64 = all_bb.get(PieceKind::Pawn(Color::White));
    /// ```
    fn bitboard_map(&self) -> HashMap<PieceKind, &u64> {
        let mut output: HashMap<PieceKind, &u64> = HashMap::new();

        output.insert(PieceKind::Pawn(Color::White), &self.w_pawns);
        output.insert(PieceKind::King(Color::White), &self.w_king);
        output.insert(PieceKind::Queen(Color::White), &self.w_queens);
        output.insert(PieceKind::Rook(Color::White), &self.w_rooks);
        output.insert(PieceKind::Bishop(Color::White), &self.w_bishops);
        output.insert(PieceKind::Knight(Color::White), &self.w_knights);

        output.insert(PieceKind::Pawn(Color::Black), &self.b_pawns);
        output.insert(PieceKind::King(Color::Black), &self.b_king);
        output.insert(PieceKind::Queen(Color::Black), &self.b_queens);
        output.insert(PieceKind::Rook(Color::Black), &self.b_rooks);
        output.insert(PieceKind::Bishop(Color::Black), &self.b_bishops);
        output.insert(PieceKind::Knight(Color::Black), &self.b_knights);

        output
    }

    /// Returns a HashMap of PieceKinds to a reference of their corresponding, mutable bitboard
    ///
    /// # Examples
    /// ```
    /// let board = create_starting_board();
    /// let all_bb = board.bitboard_map_mut();
    /// all_bb[PieceKind::Pawn(Color::White)] |= 0x1;
    /// ```
    fn bitboard_map_mut(&mut self) -> HashMap<PieceKind, &mut u64> {
        let mut output: HashMap<PieceKind, &mut u64> = HashMap::new();

        output.insert(PieceKind::Pawn(Color::White), &mut self.w_pawns);
        output.insert(PieceKind::King(Color::White), &mut self.w_king);
        output.insert(PieceKind::Queen(Color::White), &mut self.w_queens);
        output.insert(PieceKind::Rook(Color::White), &mut self.w_rooks);
        output.insert(PieceKind::Bishop(Color::White), &mut self.w_bishops);
        output.insert(PieceKind::Knight(Color::White), &mut self.w_knights);

        output.insert(PieceKind::Pawn(Color::Black), &mut self.b_pawns);
        output.insert(PieceKind::King(Color::Black), &mut self.b_king);
        output.insert(PieceKind::Queen(Color::Black), &mut self.b_queens);
        output.insert(PieceKind::Rook(Color::Black), &mut self.b_rooks);
        output.insert(PieceKind::Bishop(Color::Black), &mut self.b_bishops);
        output.insert(PieceKind::Knight(Color::Black), &mut self.b_knights);

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
    /// let board = create_starting_board();
    /// board.add_piece(&Square::new("a3"), &PieceKind::Rook(Color::White));
    /// ```
    pub fn add_piece(&mut self, square: &Square, piece: &PieceKind) {
        let mask = square.get_mask();
        self.bitboard_map_mut()
            .entry(*piece)
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
    /// let board = create_starting_board();
    /// // Playing with rook odds
    /// board.clear_piece(&Square::new("a1"));
    /// ```
    #[allow(dead_code)]
    pub fn clear_piece(&mut self, square: &Square) {
        let mask = !square.get_mask();
        for (_, bb) in self.bitboard_map_mut().iter_mut() {
            **bb &= mask;
        }
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
    /// let board = create_starting_board();
    /// // Playing with rook odds
    /// board.remove_piece(&Square::new("a1"), &PieceKind::Rook(Color::White));
    /// ```
    pub fn remove_piece(&mut self, square: &Square, piece: &PieceKind) {
        let mask = !square.get_mask();
        self.bitboard_map_mut()
            .entry(*piece)
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
    /// let board = create_starting_board();
    /// // Ply the a pawn one square forward
    /// board.make_move(Ply::new(Square::new("a2"), Square::new("a3")));
    /// ```
    pub fn make_move(&mut self, mut new_move: Ply) {
        let start_piece_kind = self.get_piece(&new_move.start).unwrap();

        self.remove_piece(&new_move.start, &start_piece_kind);
        if let Some(dest_piece_kind) = self.get_piece(&new_move.dest) {
            new_move.captured_piece = Some(dest_piece_kind);
            self.remove_piece(&new_move.dest, &dest_piece_kind);
        }
        self.add_piece(&new_move.dest, &start_piece_kind);

        self.history.push(new_move);
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
        let piece_kind = self.get_piece(&old_move.dest).unwrap();

        // Start is guaranteed to be empty since the piece we're moving back was at the start last
        // move
        self.add_piece(&old_move.start, &piece_kind);
        self.remove_piece(&old_move.dest, &piece_kind);

        if let Some(caputre_piece) = self.history.pop().unwrap().captured_piece {
            self.add_piece(&old_move.dest, &caputre_piece);
        }
    }
}

impl fmt::Display for Board {
    /// Prints out a symbolic representation of the board in an 8x8 grid.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                if let Some(piece) = self.get_piece(&Square { rank: i, file: j }) {
                    write!(f, "{}", piece)?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Creates a new board object that represents the starting board state in a normal game
pub fn create_starting_board() -> Board {
    let w_pawns = 0b0000000000000000000000000000000000000000000000001111111100000000;
    let w_king = 0b0000000000000000000000000000000000000000000000000000000000001000;
    let w_queens = 0b0000000000000000000000000000000000000000000000000000000000010000;
    let w_rooks = 0b0000000000000000000000000000000000000000000000000000000010000001;
    let w_bishops = 0b0000000000000000000000000000000000000000000000000000000000100100;
    let w_knights = 0b0000000000000000000000000000000000000000000000000000000001000010;
    let b_pawns = 0b0000000011111111000000000000000000000000000000000000000000000000;
    let b_king = 0b0000100000000000000000000000000000000000000000000000000000000000;
    let b_queens = 0b0001000000000000000000000000000000000000000000000000000000000000;
    let b_rooks = 0b1000000100000000000000000000000000000000000000000000000000000000;
    let b_bishops = 0b0010010000000000000000000000000000000000000000000000000000000000;
    let b_knights = 0b0100001000000000000000000000000000000000000000000000000000000000;

    Board::new(
        w_pawns, w_king, w_queens, w_rooks, w_bishops, w_knights, b_pawns, b_king, b_queens,
        b_rooks, b_bishops, b_knights,
    )
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece1() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(&Square::new("a1")).unwrap(),
            PieceKind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(&Square::new("h8")).unwrap(),
            PieceKind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(&Square::new("h7")).unwrap(),
            PieceKind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = create_starting_board();
        assert!(board.get_piece(&Square::new("e5")).is_none());
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_rank() {
        let board = create_starting_board();
        board.get_piece(&Square { rank: 8, file: 7 }).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_file() {
        let board = create_starting_board();
        board.get_piece(&Square { rank: 0, file: 8 }).unwrap();
    }

    #[test]
    fn test_get_moves_for_piece() {
        let board = create_starting_board();
        let moves = board.get_moves_for_piece(&Square::new("a2")); // pawn
        let correct = [
            Ply {
                start: Square::new("a2"),
                dest: Square::new("a3"),
                captured_piece: None,
            },
            Ply {
                start: Square::new("a2"),
                dest: Square::new("a4"),
                captured_piece: None,
            },
        ];

        assert_eq!(moves.unwrap(), correct);
    }

    #[test]
    #[should_panic]
    fn test_get_moves_for_piece_empty() {
        let board = create_starting_board();
        let moves = board.get_moves_for_piece(&Square::new("a3")); // Empty

        moves.unwrap();
    }

    #[test]
    fn test_get_all_moves() {
        let board = create_starting_board();
        let all_moves = board.get_all_moves();

        assert!(!all_moves.is_empty());
    }

    #[test]
    fn test_add_piece() {
        let mut board = create_starting_board();
        let square = Square::new("a3");
        board.add_piece(&square, &PieceKind::Queen(Color::White));
        assert_eq!(
            board.get_piece(&square).unwrap(),
            PieceKind::Queen(Color::White)
        );
    }

    #[test]
    fn test_clear_piece() {
        let mut board = create_starting_board();
        let square = Square::new("a2");
        board.clear_piece(&square);
        assert!(board.get_piece(&square).is_none());
    }

    #[test]
    fn test_remove_piece() {
        let mut board = create_starting_board();
        let square = Square::new("a2");

        // Should do nothing, since there is a white pawn here, not a black pawn
        board.remove_piece(&square, &PieceKind::Pawn(Color::Black));
        assert_eq!(
            board.get_piece(&square).unwrap(),
            PieceKind::Pawn(Color::White)
        );

        board.remove_piece(&square, &PieceKind::Pawn(Color::White));
        assert!(board.get_piece(&square).is_none());
    }

    #[test]
    fn test_board_display() {
        let board = create_starting_board();
        let correct =
            "♖♘♗♕♔♗♘♖\n♙♙♙♙♙♙♙♙\n--------\n--------\n--------\n--------\n♟♟♟♟♟♟♟♟\n♜♞♝♛♚♝♞♜\n";
        assert_eq!(board.to_string(), correct);
    }

    #[test]
    fn test_make_unmake_move_single() {
        let mut board = create_starting_board();
        let start = Square::new("a2");
        let dest = Square::new("a3");
        let ply = Ply::new(start, dest);

        assert!(board.get_piece(&dest).is_none());
        board.make_move(ply);
        assert_eq!(
            board.get_piece(&dest).unwrap(),
            PieceKind::Pawn(Color::White)
        );

        assert!(board.get_piece(&start).is_none());

        board.unmake_move(ply);
        assert_eq!(
            board.get_piece(&start).unwrap(),
            PieceKind::Pawn(Color::White)
        );

        assert!(board.get_piece(&dest).is_none());
    }

    #[test]
    fn test_make_unmake_move_double() {
        // Make and unmake two moves in a row
        let mut board = create_starting_board();
        let start = Square::new("a2");
        let dest1 = Square::new("a3");
        let dest2 = Square::new("a4");
        let ply1 = Ply::new(start, dest1);
        let ply2 = Ply::new(dest1, dest2);

        assert!(board.get_piece(&dest1).is_none());
        assert!(board.get_piece(&dest2).is_none());
        board.make_move(ply1);
        assert_eq!(
            board.get_piece(&dest1).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert!(board.get_piece(&start).is_none());
        assert!(board.get_piece(&dest2).is_none());

        board.make_move(ply2);
        assert_eq!(
            board.get_piece(&dest2).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert!(board.get_piece(&start).is_none());
        assert!(board.get_piece(&dest1).is_none());

        board.unmake_move(ply2);
        assert_eq!(
            board.get_piece(&dest1).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert!(board.get_piece(&dest2).is_none());
        assert!(board.get_piece(&start).is_none());

        board.unmake_move(ply1);
        assert_eq!(
            board.get_piece(&start).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert!(board.get_piece(&dest2).is_none());
        assert!(board.get_piece(&dest1).is_none());
    }

    #[test]
    fn test_make_unmake_move_capture() {
        let mut board = create_starting_board();
        let start = Square::new("a2"); // White Pawn
        let dest = Square::new("a7"); // Black Pawn
        let ply = Ply::new(start, dest);

        assert_eq!(
            board.get_piece(&start).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert_eq!(
            board.get_piece(&dest).unwrap(),
            PieceKind::Pawn(Color::Black)
        );
        board.make_move(ply);
        assert_eq!(
            board.get_piece(&dest).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert!(board.get_piece(&start).is_none());

        board.unmake_move(ply);
        assert_eq!(
            board.get_piece(&start).unwrap(),
            PieceKind::Pawn(Color::White)
        );
        assert_eq!(
            board.get_piece(&dest).unwrap(),
            PieceKind::Pawn(Color::Black)
        );
    }

    #[test]
    fn from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(create_starting_board(), Board::from_fen(fen));
    }

    #[test]
    fn from_fen_white_position1() {
        let fen = "1k1r3r/p6p/1pp1pp2/2Np1qp1/1Q1P4/2P1PP2/PP4PP/R4nK1 w - - 0 21";
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: false,
            w_queenside_castling: false,
            b_kingside_castling: false,
            b_queenside_castling: false,

            w_pawns: 271368960,
            w_king: 2,
            w_queens: 1073741824,
            w_rooks: 128,
            w_bishops: 0,
            w_knights: 137438953472,
            b_pawns: 36429096560885760,
            b_king: 4611686018427387904,
            b_queens: 17179869184,
            b_rooks: 1224979098644774912,
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

            w_kingside_castling: false,
            w_queenside_castling: false,
            b_kingside_castling: false,
            b_queenside_castling: false,

            w_pawns: 337691392,
            w_king: 1024,
            w_queens: 4294967296,
            w_rooks: 65664,
            w_bishops: 1048608,
            w_knights: 4503599627370496,
            b_pawns: 54642446545453056,
            b_king: 281474976710656,
            b_queens: 4398046511104,
            b_rooks: 8,
            b_bishops: 288230376151711744,
            b_knights: 17660905521152,
            history: Vec::new(),
        };

        assert_eq!(Board::from_fen(fen), correct);
    }
}
