use std::collections::HashMap;
use std::fmt;

mod boardbuilder;
pub mod piece;
mod ply;
mod square;

use piece::{Color, PieceKind};
use ply::Ply;
use square::Square;
use boardbuilder::BoardBuilder;

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
    pub fn new() -> BoardBuilder {
        BoardBuilder::default()
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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
    /// let board = Board::new().starting_board();
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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece1() {
        let board = Board::new().starting_board();
        assert_eq!(
            board.get_piece(&Square::new("a1")).unwrap(),
            PieceKind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = Board::new().starting_board();
        assert_eq!(
            board.get_piece(&Square::new("h8")).unwrap(),
            PieceKind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = Board::new().starting_board();
        assert_eq!(
            board.get_piece(&Square::new("h7")).unwrap(),
            PieceKind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = Board::new().starting_board();
        assert!(board.get_piece(&Square::new("e5")).is_none());
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_rank() {
        let board = Board::new().starting_board();
        board.get_piece(&Square { rank: 8, file: 7 }).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_file() {
        let board = Board::new().starting_board();
        board.get_piece(&Square { rank: 0, file: 8 }).unwrap();
    }

    #[test]
    fn test_get_moves_for_piece() {
        let board = Board::new().starting_board();
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
        let board = Board::new().starting_board();
        let moves = board.get_moves_for_piece(&Square::new("a3")); // Empty

        moves.unwrap();
    }

    #[test]
    fn test_get_all_moves() {
        let board = Board::new().starting_board();
        let all_moves = board.get_all_moves();

        assert!(!all_moves.is_empty());
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::new().starting_board();
        let square = Square::new("a3");
        board.add_piece(&square, &PieceKind::Queen(Color::White));
        assert_eq!(
            board.get_piece(&square).unwrap(),
            PieceKind::Queen(Color::White)
        );
    }

    #[test]
    fn test_clear_piece() {
        let mut board = Board::new().starting_board();
        let square = Square::new("a2");
        board.clear_piece(&square);
        assert!(board.get_piece(&square).is_none());
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::new().starting_board();
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
        let board = Board::new().starting_board();
        let correct =
            "♖♘♗♕♔♗♘♖\n♙♙♙♙♙♙♙♙\n--------\n--------\n--------\n--------\n♟♟♟♟♟♟♟♟\n♜♞♝♛♚♝♞♜\n";
        assert_eq!(board.to_string(), correct);
    }

    #[test]
    fn test_make_unmake_move_single() {
        let mut board = Board::new().starting_board();
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
        let mut board = Board::new().starting_board();
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
        let mut board = Board::new().starting_board();
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
}