use std::collections::HashMap;
use std::fmt;

pub mod piece;

use piece::{Color, Move, PieceKind, Square};

// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
pub struct Board {
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
}

impl Board {
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
    /// assert_eq!(PieceKind::Rook(Color::White), board.get_piece(Square::new(0,0));
    /// assert_eq!(None, board.get_piece(Square::new(3,0));
    /// ```
    pub fn get_piece(&self, square: &Square) -> Option<PieceKind> {
        let mask = mask_for_coord(square);
        for (kind, bb) in self.bitboard_map() {
            if (*bb & mask) >= 1 {
                return Some(kind.clone());
            }
        }
        None
    }

    /// Returns an optional list of Moves for the piece at a given square
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
    /// let movelist = board.get_moves_for_piece(Square::new(1,0));
    /// ```
    pub fn get_moves_for_piece(&self, square: &Square) -> Option<Vec<Move>> {
        let piece = self.get_piece(square);
        match piece {
            Some(p) => Some(p.get_all_moves(square)),
            None => None,
        }
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
    /// board.add_piece(&Square::new(2,0), &PieceKind::Rook(Color::White));
    /// ```
    pub fn add_piece(&mut self, square: &Square, piece: &PieceKind) {
        let mask = mask_for_coord(square);
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
    /// board.clear_piece(&Square::new(0, 0));
    /// ```
    pub fn clear_piece(&mut self, square: &Square) {
        let mask = !mask_for_coord(square);
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
    /// board.remove_piece(&Square::new(0,0), &PieceKind::Rook(Color::White));
    /// ```
    pub fn remove_piece(&mut self, square: &Square, piece: &PieceKind) {
        let mask = !mask_for_coord(square);
        self.bitboard_map_mut()
            .entry(*piece)
            .and_modify(|bb| **bb &= mask);
    }

    /// Makes a half-move on this board
    ///
    /// # Arguments
    ///
    /// * `new_move` - A Move that holds the origin and destination square of the move.
    ///
    /// # Examples
    /// ```
    /// let board = create_starting_board();
    /// // Move the a pawn one square forward
    /// board.make_move(Move::new(Square::new(1, 0), Square::new(2, 0)));
    /// ```
    pub fn make_move(&mut self, new_move: Move) {
        let piece_kind = self.get_piece(&new_move.start).unwrap();

        self.clear_piece(&new_move.dest);
        self.add_piece(&new_move.dest, &piece_kind);
        self.remove_piece(&new_move.start, &piece_kind);
    }
}

/// Returns a u64 mask filled with 0s except for a 1 in the designated square
///
/// # Arguments
///
/// * `square` - A square that indicates the desired bit to set to 1
///
/// # Examples
/// ```
/// let mask = mask_for_coord(Square::new(1,4));
/// ```
fn mask_for_coord(square: &Square) -> u64 {
    let rank_mask: u64 = 0x00000000000000FF << (8 * square.rank);
    let file_mask: u64 = 0x0101010101010101 << (8 - (square.file + 1));

    rank_mask & file_mask
}

impl fmt::Display for Board {
    /// Prints out a symbolic representation of the board in an 8x8 grid.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.get_piece(&Square::new(i, j));
                match piece {
                    Some(p) => write!(f, "{}", p)?,
                    None => write!(f, "-")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

/// Creates a new board object that represents the starting board state in a normal game
pub fn create_starting_board() -> Board {
    Board {
        w_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
        w_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
        w_queens: 0b0000000000000000000000000000000000000000000000000000000000010000,
        w_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
        w_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
        w_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
        b_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
        b_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
        b_queens: 0b0001000000000000000000000000000000000000000000000000000000000000,
        b_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
        b_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
        b_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece1() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(0, 0).unwrap(),
            PieceKind::Rook(Color::White)
        );
    }

    #[test]
    fn test_get_piece2() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(7, 7).unwrap(),
            PieceKind::Rook(Color::Black)
        );
    }

    #[test]
    fn test_get_piece3() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece(6, 7).unwrap(),
            PieceKind::Pawn(Color::Black)
        );
    }

    #[test]
    fn test_get_piece_none() {
        let board = create_starting_board();
        assert!(board.get_piece(4, 4).is_none());
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_rank() {
        let board = create_starting_board();
        board.get_piece(8, 7).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_piece_oob_file() {
        let board = create_starting_board();
        board.get_piece(0, 8).unwrap();
    }
}
