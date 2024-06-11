use super::board::{Board, Ply};
use super::evaluate::Evaluator;

const DEFAULT_DEPTH: usize = 4;

pub fn search(board: &mut Board, evaluator: &impl Evaluator, depth: Option<usize>) -> Ply {
    negamax(board, evaluator, depth.unwrap_or(DEFAULT_DEPTH))
}

fn negamax(board: &mut Board, evaluator: &impl Evaluator, depth: usize) -> Ply {
    let mut best_value = i32::MIN;
    let moves = board.get_legal_moves();
    let mut best_ply = moves[0];

    for mv in moves {
        board.make_move(mv);
        let value = negamax_helper(board, depth - 1, evaluator).saturating_neg();
        if value > best_value {
            best_value = value;
            best_ply = mv;
        }
        board.unmake_move();
    }

    println!("Best value: {best_value}");
    best_ply
}

fn negamax_helper(board: &mut Board, depth: usize, evaluator: &impl Evaluator) -> i32 {
    if depth == 0 {
        return evaluator.evaluate(board);
    }

    let mut best_value = i32::MIN;
    let moves = board.get_legal_moves();

    for mv in moves {
        board.make_move(mv);
        let value = negamax_helper(board, depth - 1, evaluator).saturating_neg();
        best_value = best_value.max(value);
        board.unmake_move();
    }

    best_value
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::board::BoardBuilder;
    use crate::evaluate::simple_evaluator::SimpleEvaluator;
    use test::Bencher;

    #[bench]
    fn bench_search_depth_4(bencher: &mut Bencher) {
        let mut board = BoardBuilder::construct_starting_board();
        let evaluator = SimpleEvaluator::new();
        bencher.iter(|| search(&mut board, &evaluator, Some(4)));
    }
}
