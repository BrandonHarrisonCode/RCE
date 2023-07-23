use super::*;

#[derive(Clone, PartialEq)]
pub struct Bishop;

const WHITE_SYMBOL: &str = "♝";
const BLACK_SYMBOL: &str = "♗";

impl Eq for Bishop {}

impl Piece for Bishop {
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
