use super::Evaluator;
use crate::board::piece::Kind;
use crate::board::square::Square;
use crate::board::Board;

#[derive(Clone)]
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    const KING_VALUE: i64 = i32::MAX as i64;
    const QUEEN_VALUE: i64 = 900;
    const ROOK_VALUE: i64 = 500;
    const BISHOP_VALUE: i64 = 300;
    const KNIGHT_VALUE: i64 = 300;
    const PAWN_VALUE: i64 = 100;

    pub const fn new() -> Self {
        Self {}
    }
}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, board: &mut Board) -> i64 {
        let mut score: i64 = 0;

        for square in 0..64u8 {
            if let Some(piece) = board.get_piece(Square::from(square)) {
                let piece_value = match piece {
                    Kind::King(_) => Self::KING_VALUE,
                    Kind::Queen(_) => Self::QUEEN_VALUE,
                    Kind::Rook(_) => Self::ROOK_VALUE,
                    Kind::Bishop(_) => Self::BISHOP_VALUE,
                    Kind::Knight(_) => Self::KNIGHT_VALUE,
                    Kind::Pawn(_) => Self::PAWN_VALUE,
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
