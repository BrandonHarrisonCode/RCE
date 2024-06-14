use super::board::{Board, Ply};
use super::evaluate::Evaluator;
use parking_lot::{Mutex, RwLock};

const DEFAULT_DEPTH: usize = 5;

pub mod limits;

use limits::SearchLimits;

#[allow(dead_code)]
pub struct Search<T: Evaluator> {
    board: Mutex<Board>,
    evaluator: T,
    limits: Mutex<SearchLimits>,
    best_move: Option<Ply>,
    running: RwLock<bool>,
}

impl<T: Evaluator> Search<T> {
    pub fn new(board: &Board, evaluator: &T, limits: Option<SearchLimits>) -> Self {
        Self {
            board: Mutex::new(board.clone()),
            evaluator: evaluator.clone(),
            limits: Mutex::new(limits.unwrap_or_default()),
            best_move: None,
            running: RwLock::new(true),
        }
    }

    #[allow(dead_code)]
    pub const fn get_best_move(&self) -> Option<Ply> {
        self.best_move
    }

    pub fn stop(&self) {
        let mut running = self.running.write();
        *running = false;
    }

    pub fn start(&self) {
        let mut running = self.running.write();
        *running = true;
    }

    pub fn search(&self, depth: Option<usize>) -> Ply {
        self.alpha_beta_start(depth.unwrap_or(DEFAULT_DEPTH))
    }

    pub fn set_limits(&self, limits: SearchLimits) {
        let mut lock = self.limits.lock();
        *lock = limits;
    }

    const fn check_limits(&self) -> bool {
        false
    }

    fn alpha_beta_start(&self, depth: usize) -> Ply {
        let mut best_value = i32::MIN;
        let mut board = self.board.lock();
        let moves = board.get_legal_moves();
        drop(board);
        let mut best_ply = moves[0];

        for mv in moves {
            let mut board = self.board.lock();
            board.make_move(mv);
            drop(board);
            let value = self.alpha_beta_min(i32::MIN, i32::MAX, depth - 1);
            if value > best_value {
                best_value = value;
                best_ply = mv;
            }
            let mut board = self.board.lock();
            board.unmake_move();
            drop(board);
        }

        best_ply
    }

    fn alpha_beta_max(&self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        let running_reader = self.running.read();
        let mut board = self.board.lock();

        if depth == 0 || !*running_reader || self.check_limits() {
            return self.evaluator.evaluate(&board);
        }
        drop(running_reader);

        let moves = board.get_legal_moves();
        drop(board);

        for mv in moves {
            let mut board = self.board.lock();
            board.make_move(mv);
            drop(board);

            let score = self.alpha_beta_min(alpha, beta, depth - 1);

            let mut board = self.board.lock();
            board.unmake_move();
            drop(board);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn alpha_beta_min(&self, alpha: i32, mut beta: i32, depth: usize) -> i32 {
        let running_reader = self.running.read();
        let mut board = self.board.lock();

        if depth == 0 || !*running_reader || self.check_limits() {
            return self.evaluator.evaluate(&board);
        }
        drop(running_reader);

        let moves = board.get_legal_moves();
        drop(board);

        for mv in moves {
            let mut board = self.board.lock();
            board.make_move(mv);
            drop(board);

            let score = self.alpha_beta_max(alpha, beta, depth - 1);

            let mut board = self.board.lock();
            board.unmake_move();
            drop(board);

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
        let search = Search::new(&board, &evaluator, None);
        bencher.iter(|| search.search(Some(4)));
    }
}
