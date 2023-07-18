use std::collections::HashMap;
use std::fmt;

mod piece;

use piece::{Color, PieceKind};

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
    // Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
    fn get_piece_at_coords(&self, rank: u8, file: u8) -> Option<PieceKind> {
        let rank_mask: u64 = 0x00000000000000FF << 8 * (rank - 1);
        let file_mask: u64 = 0x0101010101010101 << (8 - file);
        for (kind, bb) in self.bitboard_map() {
            if (bb & rank_mask & file_mask) >= 1 {
                return Some(kind.clone());
            }
        }
        None
    }

    fn bitboard_map(&self) -> HashMap<PieceKind, u64> {
        let mut output: HashMap<PieceKind, u64> = HashMap::new();

        output.insert(PieceKind::Pawn(Color::White), self.w_pawns);
        output.insert(PieceKind::King(Color::White), self.w_king);
        output.insert(PieceKind::Queen(Color::White), self.w_queens);
        output.insert(PieceKind::Rook(Color::White), self.w_rooks);
        output.insert(PieceKind::Bishop(Color::White), self.w_bishops);
        output.insert(PieceKind::Knight(Color::White), self.w_knights);

        output.insert(PieceKind::Pawn(Color::Black), self.b_pawns);
        output.insert(PieceKind::King(Color::Black), self.b_king);
        output.insert(PieceKind::Queen(Color::Black), self.b_queens);
        output.insert(PieceKind::Rook(Color::Black), self.b_rooks);
        output.insert(PieceKind::Bishop(Color::Black), self.b_bishops);
        output.insert(PieceKind::Knight(Color::Black), self.b_knights);

        output
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (1..=8).rev() {
            for j in 1..=8 {
                let piece = self.get_piece_at_coords(i, j);
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
    fn test_get_piece_at_coords1() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece_at_coords(1, 1).unwrap(),
            Piece::new(Color::White, PieceKind::Rook)
        );
    }

    #[test]
    fn test_get_piece_at_coords2() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece_at_coords(8, 8).unwrap(),
            Piece::new(Color::Black, PieceKind::Rook)
        );
    }

    #[test]
    fn test_get_piece_at_coords3() {
        let board = create_starting_board();
        assert_eq!(
            board.get_piece_at_coords(7, 8).unwrap(),
            Piece::new(Color::Black, PieceKind::Pawn)
        );
    }

    #[test]
    fn test_get_piece_at_coords_none() {
        let board = create_starting_board();
        assert!(board.get_piece_at_coords(4, 4).is_none());
    }

    #[test]
    #[should_panic]
    fn test_get_piece_at_coords_oob_rank() {
        let board = create_starting_board();
        board.get_piece_at_coords(9, 8).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_piece_at_coords_oob_file() {
        let board = create_starting_board();
        board.get_piece_at_coords(0, 9).unwrap();
    }
}
