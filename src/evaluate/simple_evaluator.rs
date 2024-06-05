use super::Evaluator;
use crate::board::piece::Kind;
use crate::board::square::Square;
use crate::board::Board;
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    const KING_VALUE: i32 = i16::MAX as i32;
    const QUEEN_VALUE: i32 = 9;
    const ROOK_VALUE: i32 = 5;
    const BISHOP_VALUE: i32 = 3;
    const KNIGHT_VALUE: i32 = 3;
    const PAWN_VALUE: i32 = 1;

    pub fn new() -> Self {
        SimpleEvaluator {}
    }
}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score: i32 = 0;

        for square in 0..64u8 {
            if let Some(piece) = board.get_piece(Square::from(square)) {
                let piece_value = match piece {
                    Kind::King(_) => SimpleEvaluator::KING_VALUE,
                    Kind::Queen(_) => SimpleEvaluator::QUEEN_VALUE,
                    Kind::Rook(_) => SimpleEvaluator::ROOK_VALUE,
                    Kind::Bishop(_) => SimpleEvaluator::BISHOP_VALUE,
                    Kind::Knight(_) => SimpleEvaluator::KNIGHT_VALUE,
                    Kind::Pawn(_) => SimpleEvaluator::PAWN_VALUE,
                };

                if piece.get_color() == board.current_turn {
                    score = score.saturating_add(piece_value);
                } else {
                    score = score.saturating_sub(piece_value);
                }
            }
        }

        score
    }
}
