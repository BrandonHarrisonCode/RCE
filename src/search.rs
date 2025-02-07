use crate::board::piece::Color;

use super::board::{Board, Ply};
use super::evaluate::Evaluator;
use logger::Logger;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub mod limits;
mod logger;

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
    movetime: u128,
}

impl<T: Evaluator> Logger for Search<T> {
    fn log(&self, message: &str) {
        println!("{message}");
    }
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

    /// Logs the UCI output
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth of the search
    /// * `time_elapsed_in_ms` - The time elapsed in milliseconds
    /// * `best_value` - The best value found by the search
    /// * `best_ply` - The best move found by the search
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.log_uci(3, 1000, 0, Ply::new(0, 0, 0, 0));
    /// ```
    fn log_uci_info(&self, depth: usize, time_elapsed_in_ms: u128, best_value: i64, best_ply: Ply) {
        let score = match best_value {
            i64::MIN | NEGMAX => String::from("mate -1"),
            i64::MAX => String::from("mate 1"),
            _ => format!("cp {best_value}"),
        };
        self.log(
            format!("info depth {depth} time {time_elapsed_in_ms} score {score} pv {best_ply}")
                .as_str(),
        );
    }

    #[allow(dead_code)]
    /// Returns the best move found by the search so far
    ///
    /// # Returns
    ///
    /// * `Option<Ply>` - The best move found by the search so far, if one has been found
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.search(Some(3));
    /// let best_move = search.get_best_move();
    /// ```
    pub const fn get_best_move(&self) -> Option<Ply> {
        self.best_move
    }

    /// Returns the `AtomicBool` that is used to determine if the search should continue
    ///
    /// # Returns
    ///
    /// * `Arc<AtomicBool>` - The `AtomicBool` that is used to determine if the search should continue
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let running = search.get_running();
    /// ```
    pub fn get_running(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }

    /// Returns a boolean determining if the search is still running
    ///
    /// # Returns
    ///
    /// * `bool` - A boolean determining if the search is still running
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let running = search.check_running();
    /// ```
    pub fn check_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Checks if the search has exceeded any of the limits
    ///
    /// # Returns
    ///
    /// * `bool` - A boolean determining if the search has exceeded any of the limits
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let limits_exceeded = search.check_limits();
    /// ```
    fn limits_exceeded(&self) -> bool {
        if let Some(depth) = self.limits.depth {
            if u128::from(self.depth) >= depth {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }
        if let Some(nodes) = self.limits.nodes {
            if u128::from(self.nodes) >= nodes {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }
        if let Some(movetime) = self.limits.movetime {
            if self.movetime >= movetime {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }

        false
    }

    /// Checks if the search has exceeded limits related to time restrictions.
    ///
    /// # Returns
    ///
    /// * `bool` - A boolean determining if the search has exceeded any of the time-related limits
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let time_limits_exceeded = search.time_limits_exceeded();
    /// ```
    pub fn time_limits_exceeded(&self, start: Instant) -> bool {
        let duration = start.elapsed();
        let time_elapsed_in_ms = duration.as_millis();
        time_elapsed_in_ms >= self.limits.movetime.unwrap_or(u128::MAX)
            || ([
                self.limits.white_time,
                self.limits.white_increment,
                self.limits.black_time,
                self.limits.black_increment,
            ]
            .iter()
            .any(Option::is_some)
                && time_elapsed_in_ms >= self.limits.time_management_timer.unwrap_or(u128::MAX))
    }

    /// Initializes the search and returns the best move found
    ///
    /// # Arguments
    ///
    /// * `depth` - An optional `usize` that determines the depth of the search
    ///
    /// # Returns
    ///
    /// * `Ply` - The best move found by the search
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let best_move = search.search(Some(3));
    /// ```
    pub fn search(&mut self, max_depth: Option<usize>) {
        self.iter_deep(max_depth);
    }

    /// Iterates through the search at increasing depths until the search is stopped or the maximum depth is reached
    /// or the movetime limit is exceeded
    ///
    /// # Arguments
    ///
    /// * `max_depth` - An optional `usize` that determines the maximum depth to search to
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.iter_deep(Some(3));
    /// ```
    fn iter_deep(&mut self, max_depth: Option<usize>) {
        let start = Instant::now();
        // Uses a heuristic to determine the maximum time to spend on a move
        self.limits.time_management_timer = match self.board.current_turn {
            Color::White => {
                self.limits.white_time.unwrap_or(0) / 20
                    + self.limits.white_increment.unwrap_or(0) / 2
            }
            .into(),
            Color::Black => {
                self.limits.black_time.unwrap_or(0) / 20
                    + self.limits.black_increment.unwrap_or(0) / 2
            }
            .into(),
        };

        for depth in 1..max_depth.unwrap_or(usize::MAX) {
            let current_best_move = self.alpha_beta_start(depth, start);

            if !self.check_running() {
                break;
            }

            if self.time_limits_exceeded(start) {
                break;
            }
            self.best_move = Some(current_best_move);
        }

        self.log(format!("bestmove {}", self.best_move.unwrap()).as_str());
    }

    /// Initializes the alpha-beta search and returns the best move found
    ///
    /// # Arguments
    ///
    /// * `depth` - A `usize` that determines the depth of the search
    ///
    /// # Returns
    ///
    /// * `Ply` - The best move found by the search
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let best_move = search.alpha_beta_start(3);
    /// ```
    fn alpha_beta_start(&mut self, depth: usize, start: Instant) -> Ply {
        let mut best_value = i64::MIN;
        let moves = self.board.get_legal_moves();

        let mut best_ply = moves[0];

        for mv in moves {
            self.board.make_move(mv);

            let value = self
                .alpha_beta(i64::MIN, i64::MAX, depth - 1, start)
                .saturating_neg();
            if value > best_value {
                best_value = value;
                best_ply = mv;
            }
            self.board.unmake_move();
        }

        let duration = start.elapsed();
        let time_elapsed_in_ms = duration.as_millis();
        self.log_uci_info(depth, time_elapsed_in_ms, best_value, best_ply);

        best_ply
    }

    /// The alpha-beta search algorithm
    ///
    /// # Arguments
    ///
    /// * `alpha` - The best value for the maximizing player found so far
    /// * `beta` - The best value for the minimizing player found so far
    /// * `depthleft` - The depth left to search
    ///
    /// # Returns
    ///
    /// * `i64` - The score of the "best" position
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let score = search.alpha_beta(i64::MIN, i64::MAX, 3);
    /// ```
    fn alpha_beta(&mut self, mut alpha: i64, beta: i64, depthleft: usize, start: Instant) -> i64 {
        if depthleft == 0
            || !self.check_running()
            || self.limits_exceeded()
            || self.time_limits_exceeded(start)
        {
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
                .alpha_beta(
                    beta.saturating_neg(),
                    alpha.saturating_neg(),
                    depthleft - 1,
                    start,
                )
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

    #[test]
    fn test_get_best_move() {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        assert!(search.get_best_move().is_none());
        search.search(Some(3));
        let best_move = search.get_best_move();
        assert!(best_move.is_some());
    }

    #[test]
    fn test_log_uci() {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let search = Search::new(&board, &evaluator, None);
        search.log_uci_info(3, 1500, 10, Ply::default());
    }

    #[test]
    fn test_get_running() {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let search = Search::new(&board, &evaluator, None);
        assert!(search.check_running());
        search.get_running().store(false, Ordering::Relaxed);
        assert!(!search.check_running());
    }

    #[test]
    fn test_check_limits() {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        assert!(!search.limits_exceeded());
        search.limits.depth = Some(3);
        assert!(!search.limits_exceeded());
        search.depth = 3;
        assert!(search.limits_exceeded());
        search.limits.depth = None;
        search.limits.nodes = Some(100);
        assert!(!search.limits_exceeded());
        search.nodes = 100;
        assert!(search.limits_exceeded());
        search.limits.nodes = None;
        search.limits.movetime = Some(1000);
        assert!(!search.limits_exceeded());
        search.movetime = 1000;
        assert!(search.limits_exceeded());
    }

    #[test]
    fn test_alpha_beta() {
        let board = BoardBuilder::construct_starting_board().build();
        let evaluator = SimpleEvaluator::new();
        let mut search = Search::new(&board, &evaluator, None);
        let score = search.alpha_beta(i64::MIN, i64::MAX, 4, Instant::now());
        assert_eq!(score, 0)
    }

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
