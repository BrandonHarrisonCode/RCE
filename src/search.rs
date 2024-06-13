use super::board::{Board, Ply};
use super::evaluate::Evaluator;

const DEFAULT_DEPTH: usize = 5;

pub mod limits;

use limits::SearchLimits;

#[allow(dead_code)]
pub struct Search<T: Evaluator> {
    board: Board,
    evaluator: T,
    limits: SearchLimits,
    best_move: Option<Ply>,
    running: bool,
}

impl<T: Evaluator> Search<T> {
    pub fn new(board: &Board, evaluator: &T, limits: Option<SearchLimits>) -> Self {
        Self {
            board: board.clone(),
            evaluator: evaluator.clone(),
            limits: limits.unwrap_or_default(),
            best_move: None,
            running: false,
        }
    }

    #[allow(dead_code)]
    pub const fn get_best_move(&self) -> Option<Ply> {
        self.best_move
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn search(&mut self, depth: Option<usize>) -> Ply {
        self.alpha_beta_start(depth.unwrap_or(DEFAULT_DEPTH))
    }

    fn alpha_beta_start(&mut self, depth: usize) -> Ply {
        let mut best_value = i32::MIN;
        let moves = self.board.get_legal_moves();
        let mut best_ply = moves[0];

        for mv in moves {
            self.board.make_move(mv);
            let value = self.alpha_beta_min(i32::MIN, i32::MAX, depth - 1);
            if value > best_value {
                best_value = value;
                best_ply = mv;
            }
            self.board.unmake_move();
        }

        best_ply
    }

    fn alpha_beta_max(&mut self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        if depth == 0 {
            return self.evaluator.evaluate(&self.board);
        }

        let moves = self.board.get_legal_moves();

        for mv in moves {
            self.board.make_move(mv);
            let score = self.alpha_beta_min(alpha, beta, depth - 1);
            self.board.unmake_move();
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn alpha_beta_min(&mut self, alpha: i32, mut beta: i32, depth: usize) -> i32 {
        if depth == 0 {
            return -self.evaluator.evaluate(&self.board);
        }

        let moves = self.board.get_legal_moves();

        for mv in moves {
            self.board.make_move(mv);
            let score = self.alpha_beta_max(alpha, beta, depth - 1);
            self.board.unmake_move();
            if score <= alpha {
                return alpha;
            }
            if score < beta {
                beta = score;
            }
        }

        beta
    }
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
        let board = BoardBuilder::construct_starting_board();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(4)));
    }
}
