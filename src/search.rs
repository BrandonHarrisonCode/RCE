use super::board::{Board, Ply};
use super::evaluate::Evaluator;

const DEFAULT_DEPTH: usize = 5;

pub fn search(board: &mut Board, evaluator: &impl Evaluator, depth: Option<usize>) -> Ply {
    alpha_beta_start(board, evaluator, depth.unwrap_or(DEFAULT_DEPTH))
}

fn alpha_beta_start(board: &mut Board, evaluator: &impl Evaluator, depth: usize) -> Ply {
    let mut best_value = i32::MIN;
    let moves = board.get_legal_moves();
    let mut best_ply = moves[0];

    for mv in moves {
        board.make_move(mv);
        let value = alpha_beta_min(board, evaluator, i32::MIN, i32::MAX, depth - 1);
        if value > best_value {
            best_value = value;
            best_ply = mv;
        }
        board.unmake_move();
    }

    best_ply
}

fn alpha_beta_max(
    board: &mut Board,
    evaluator: &impl Evaluator,
    mut alpha: i32,
    beta: i32,
    depth: usize,
) -> i32 {
    if depth == 0 {
        return evaluator.evaluate(board);
    }

    let moves = board.get_legal_moves();

    for mv in moves {
        board.make_move(mv);
        let score = alpha_beta_min(board, evaluator, alpha, beta, depth - 1);
        board.unmake_move();
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

fn alpha_beta_min(
    board: &mut Board,
    evaluator: &impl Evaluator,
    alpha: i32,
    mut beta: i32,
    depth: usize,
) -> i32 {
    if depth == 0 {
        return -evaluator.evaluate(board);
    }

    let moves = board.get_legal_moves();

    for mv in moves {
        board.make_move(mv);
        let score = alpha_beta_max(board, evaluator, alpha, beta, depth - 1);
        board.unmake_move();
        if score <= alpha {
            return alpha;
        }
        if score < beta {
            beta = score;
        }
    }

    beta
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
