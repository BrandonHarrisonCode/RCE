#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use std::fmt;

struct Board {
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
    fn bitboard_map(&self) -> HashMap<Piece, u64> {
        HashMap::from([
            (build_piece(Color::White, PieceKind::Pawn), self.w_pawns),
            (build_piece(Color::White, PieceKind::King), self.w_king),
            (build_piece(Color::White, PieceKind::Queen), self.w_queens),
            (build_piece(Color::White, PieceKind::Rook), self.w_rooks),
            (build_piece(Color::White, PieceKind::Bishop), self.w_bishops),
            (build_piece(Color::White, PieceKind::Knight), self.w_knights),
            (build_piece(Color::Black, PieceKind::Pawn), self.b_pawns),
            (build_piece(Color::Black, PieceKind::King), self.b_king),
            (build_piece(Color::Black, PieceKind::Queen), self.b_queens),
            (build_piece(Color::Black, PieceKind::Rook), self.b_rooks),
            (build_piece(Color::Black, PieceKind::Bishop), self.b_bishops),
            (build_piece(Color::Black, PieceKind::Knight), self.b_knights),
        ])
    }
}

fn build_piece(color: Color, kind: PieceKind) -> Piece {
    Piece { color, kind }
}

#[derive(Clone, PartialEq, Hash)]
struct Piece {
    color: Color,
    kind: PieceKind,
}
impl Eq for Piece {}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.color, self.kind)
    }
}

#[derive(Clone, PartialEq, Hash, Display)]
enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Hash, Display)]
enum PieceKind {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

fn build_starting_board() -> Board {
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

// Starts at bottom left corner of a chess board (a1), wrapping left to right on each row
fn get_piece_at_coords(board: &Board, rank: u8, file: u8) -> Option<Piece> {
    let rank_mask: u64 = 0x00000000000000FF << 8 * (rank - 1);
    let file_mask: u64 = 0x0101010101010101 << (8 - file);
    for (kind, bb) in board.bitboard_map() {
        if (bb & rank_mask & file_mask) >= 1 {
            return Some(kind.clone());
        }
    }
    None
}

fn main() {
    let board = build_starting_board();

    for i in 1..=8 {
        for j in 1..=8 {
            let piece = get_piece_at_coords(&board, i, j);
            if let Some(p) = piece {
                println!("Piece at rank {}, file {} = {}", i, j, p,);
            };
        }
    }
}
