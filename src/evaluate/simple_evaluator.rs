use super::Evaluator;
use crate::board::piece::Kind;
use crate::board::square::Square;
use crate::board::Board;
use crate::search::Score;

/// A simple evaluator that assigns a value to each piece and sums them up.
#[derive(Clone)]
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    const KING_VALUE: Score = Score::MAX / 2;
    const QUEEN_VALUE: Score = 900;
    const ROOK_VALUE: Score = 500;
    const BISHOP_VALUE: Score = 300;
    const KNIGHT_VALUE: Score = 300;
    const PAWN_VALUE: Score = 100;
}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, board: &mut Board) -> Score {
        let mut score: Score = 0;

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
