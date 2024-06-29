use super::board::{Board, Ply};
use super::evaluate::Evaluator;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

const DEFAULT_DEPTH: usize = 6;

pub mod limits;

use limits::SearchLimits;

const NEGMAX: i64 = -i64::MAX;
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

            let value = self
                .alpha_beta(i64::MIN, i64::MAX, depth - 1)
                .saturating_neg();
            if value > best_value {
                best_value = value;
                best_ply = mv;
            }
            self.board.unmake_move();
        }

        let duration = start.elapsed();
        let time_elapsed_in_ms = duration.as_millis();
        match best_value {
            i64::MIN | NEGMAX => {
                println!(
                    "info depth {depth} time {time_elapsed_in_ms} score mate -1 pv {best_ply}"
                );
            }
            i64::MAX => {
                println!("info depth {depth} time {time_elapsed_in_ms} score mate 1 pv {best_ply}");
            }
            _ => {
                println!(
                    "info depth {depth} time {time_elapsed_in_ms} score cp {best_value} pv {best_ply}",
                );
            }
        }

        self.best_move = Some(best_ply);

        best_ply
    }

    fn alpha_beta(&mut self, mut alpha: i64, beta: i64, depthleft: usize) -> i64 {
        if depthleft == 0 || !self.check_running() || self.check_limits() {
            return self.evaluator.evaluate(&mut self.board);
        }

        let moves = self.board.get_legal_moves();
        if moves.is_empty() {
            if self.board.is_in_check(self.board.current_turn) {
                return i64::MIN; // Checkmate
            }
            return 0; // Stalemate
        }

        for mv in moves {
            self.board.make_move(mv);
            let score = self
                .alpha_beta(beta.saturating_neg(), alpha.saturating_neg(), depthleft - 1)
                .saturating_neg();
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
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(3)));
    }

    #[bench]
    fn bench_search_depth_4(bencher: &mut Bencher) {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(4)));
    }
}
