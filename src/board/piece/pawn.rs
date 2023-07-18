use super::*;

#[derive(Clone, PartialEq)]
pub struct Pawn;

const WHITE_SYMBOL: &str = "♟";
const BLACK_SYMBOL: &str = "♙";

impl Eq for Pawn {}

impl Piece for Pawn {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    // Advances 1 square forward
    // Advances 2 squares forward if on second rank
    // Takes diagonally forward
    // En passant
    // Promotion
    fn get_all_moves(rank: u8, file: u8) -> Vec<Move> {
        let start: Square = Square::new(rank, file);

        let mut output: Vec<Move> = Vec::new();
        output.push(Move::new(
            start.clone(),
            start + Direction::North.unit_square(),
        ));
        output
    }
}
