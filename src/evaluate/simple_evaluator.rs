use super::Evaluator;
use crate::board::piece::Kind;
use crate::board::Board;
use crate::search::Score;

/// A simple evaluator that assigns a value to each piece and sums them up.
#[derive(Clone)]
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    const QUEEN_VALUE: Score = 900;
    const ROOK_VALUE: Score = 500;
    const BISHOP_VALUE: Score = 300;
    const KNIGHT_VALUE: Score = 300;
    const PAWN_VALUE: Score = 100;
}

impl Evaluator for SimpleEvaluator {
    #[allow(clippy::cast_possible_truncation)]
    fn evaluate(&self, board: &mut Board) -> Score {
        let mut score: Score = 0;

        for (kind, value) in [
            (Kind::Queen(board.current_turn), Self::QUEEN_VALUE),
            (Kind::Rook(board.current_turn), Self::ROOK_VALUE),
            (Kind::Bishop(board.current_turn), Self::BISHOP_VALUE),
            (Kind::Knight(board.current_turn), Self::KNIGHT_VALUE),
            (Kind::Pawn(board.current_turn), Self::PAWN_VALUE),
        ] {
            score = score.saturating_add(board.get_piece_count(kind) as i16 * value);
        }

        for (kind, value) in [
            (
                Kind::Queen(board.current_turn.opposite()),
                Self::QUEEN_VALUE,
            ),
            (Kind::Rook(board.current_turn.opposite()), Self::ROOK_VALUE),
            (
                Kind::Bishop(board.current_turn.opposite()),
                Self::BISHOP_VALUE,
            ),
            (
                Kind::Knight(board.current_turn.opposite()),
                Self::KNIGHT_VALUE,
            ),
            (Kind::Pawn(board.current_turn.opposite()), Self::PAWN_VALUE),
        ] {
            score = score.saturating_sub(board.get_piece_count(kind) as i16 * value);
        }

        score
    }
}
