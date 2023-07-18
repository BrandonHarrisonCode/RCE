use super::*;

#[derive(Clone, PartialEq)]
pub struct Bishop;

const WHITE_SYMBOL: &str = "♝";
const BLACK_SYMBOL: &str = "♗";

impl Eq for Bishop {}

impl Piece for Bishop {
    fn get_all_moves<'a>(&self, _rank: u8, _file: u8) -> &'a Vec<u64> {
        todo!();
    }
}
