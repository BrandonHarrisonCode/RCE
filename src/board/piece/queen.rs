use super::*;

#[derive(Clone, PartialEq)]
pub struct Queen;

const WHITE_SYMBOL: &str = "♛";
const BLACK_SYMBOL: &str = "♕";

impl Eq for Queen {}

impl Piece for Queen {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_all_moves<'a>(&self, _rank: u8, _file: u8) -> &'a Vec<u64> {
        todo!();
    }
}
