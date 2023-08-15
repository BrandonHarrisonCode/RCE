use super::*;

#[derive(Clone, PartialEq)]
pub struct King;

const WHITE_SYMBOL: &str = "♚";
const BLACK_SYMBOL: &str = "♔";

impl Eq for King {}

impl Piece for King {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(_square: &Square) -> Vec<Move> {
        Vec::new()
    }
}
