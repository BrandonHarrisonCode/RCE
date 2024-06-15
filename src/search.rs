use super::board::{Board, Ply};
use super::evaluate::Evaluator;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const DEFAULT_DEPTH: usize = 4;

pub mod limits;

use limits::SearchLimits;

#[allow(dead_code)]
pub struct Search<T: Evaluator> {
    board: Board,
    evaluator: T,
    limits: SearchLimits,
    best_move: Option<Ply>,
    running: Arc<AtomicBool>,

    depth: u64,
    nodes: u64,
    movetime: u64,
}

impl<T: Evaluator> Search<T> {
    pub fn new(board: &Board, evaluator: &T, limits: Option<SearchLimits>) -> Self {
        Self {
            board: board.clone(),
            evaluator: evaluator.clone(),
            limits: limits.unwrap_or_default(),
            best_move: None,
            running: Arc::new(AtomicBool::new(true)),

            depth: 0,
            nodes: 0,
            movetime: 0,
        }
    }

    #[allow(dead_code)]
    pub const fn get_best_move(&self) -> Option<Ply> {
        self.best_move
    }

    pub fn get_running(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }

    pub fn check_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn search(&mut self, depth: Option<usize>) -> Ply {
        self.alpha_beta_start(depth.unwrap_or(DEFAULT_DEPTH))
    }

    const fn check_limits(&self) -> bool {
        if let Some(depth) = self.limits.depth {
            if self.depth >= depth {
                return true;
            }
        }
        if let Some(nodes) = self.limits.nodes {
            if self.nodes >= nodes {
                return true;
            }
        }
        if let Some(movetime) = self.limits.movetime {
            if self.movetime >= movetime {
                return true;
            }
        }

        false
    }

    fn alpha_beta_start(&mut self, depth: usize) -> Ply {
        let start = Instant::now();
        let mut best_value = i64::MIN;
        let moves = self.board.get_legal_moves();
        let mut best_ply = moves[0];

        for mv in moves {
            self.board.make_move(mv);

            let value = self.alpha_beta_min(i64::MIN, i64::MAX, depth - 1);
            if value > best_value {
                best_value = value;
                best_ply = mv;
            }
            self.board.unmake_move();
        }

        let duration = start.elapsed();
        let time_elapsed_in_ms = duration.as_millis();
        println!(
            "info depth {depth} time {time_elapsed_in_ms} score cp {best_value} pv {best_ply}",
        );
        best_ply
    }

    fn alpha_beta_max(&mut self, mut alpha: i64, beta: i64, depth: usize) -> i64 {
        if depth == 0 || !self.check_running() || self.check_limits() {
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

    fn alpha_beta_min(&mut self, alpha: i64, mut beta: i64, depth: usize) -> i64 {
        if depth == 0 || !self.check_running() || self.check_limits() {
            return self.evaluator.evaluate(&self.board);
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
    fn bench_search_depth_3(bencher: &mut Bencher) {
        let board = BoardBuilder::construct_starting_board();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(3)));
    }

    #[bench]
    fn bench_search_depth_4(bencher: &mut Bencher) {
        let board = BoardBuilder::construct_starting_board();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(4)));
    }
}
