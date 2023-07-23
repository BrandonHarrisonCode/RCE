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

    /// [X] Advances 1 square forward
    /// [ ] Advances 2 squares forward if on second rank
    /// [ ] Takes diagonally forward
    /// [ ] En passant
    /// [ ] Promotion
    fn get_all_moves(square: &Square) -> Vec<Move> {
        let mut output: Vec<Move> = Vec::new();
        output.push(Move::new(
            square.clone(),
            square.clone() + Direction::North.unit_square(),
        ));
        output
    }
}
