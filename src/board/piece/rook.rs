use super::*;

#[derive(Clone, PartialEq)]
pub struct Rook;

const WHITE_SYMBOL: &str = "♜";
const BLACK_SYMBOL: &str = "♖";

impl Eq for Rook {}

impl Piece for Rook {
    fn get_all_moves<'a>(&self, _rank: u8, _file: u8) -> &'a Vec<u64> {
        todo!();
    }
}
