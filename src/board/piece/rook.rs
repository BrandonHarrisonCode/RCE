use super::*;

#[derive(Clone, PartialEq)]
pub struct Rook;

const WHITE_SYMBOL: &str = "♜";
const BLACK_SYMBOL: &str = "♖";

impl Eq for Rook {}

impl Piece for Rook {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_all_moves(_square: &Square) -> Vec<Move> {
        todo!();
    }
}
