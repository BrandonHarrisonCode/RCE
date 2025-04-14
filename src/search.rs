mod info;
pub mod limits;
mod move_orderer;

use super::evaluate::Evaluator;
use crate::board::{
    piece::Color,
    transposition_table::{Bounds, TTEntry, TRANSPOSITION_TABLE},
    Board, Ply,
};

use crate::logger::Logger;
use info::Info;
use limits::SearchLimits;
use move_orderer::MoveOrderer;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Instant;

pub type Depth = u8;
pub type NodeCount = u64;
pub type Millisecond = u128;

const NEGMAX: i64 = -i64::MAX; // i64::MIN + 1

/// The `Search` struct is responsible for performing the actual search on a board.
/// It uses iterdeep with negamax to search the best move for the current player.
/// It also uses a transposition table to store previously evaluated positions.
///
/// # Fields
///
/// * `board` - The current board state.
/// * `evaluator` - The evaluator used to evaluate the board.
/// * `limits` - The search limits.
/// * `best_move` - The best move found by the search.
/// * `running` - A thread-safe boolean that indicates if the search is still running.
///
/// * `depth` - The current depth of the search.
/// * `nodes` - The number of nodes searched.
/// * `movetime` - The time spent searching.
///
/// # Example
/// ```
/// let board = BoardBuilder::construct_starting_board().build();
/// let evaluator = SimpleEvaluator::new();
/// let mut search = Search::new(&board, &evaluator, None);
///
/// search.search(Some(3));
/// let best_move = search.get_best_move();
/// assert!(best_move.is_some());
/// ```
pub struct Search {
    pub running: Arc<AtomicBool>,
    board: Board,
    original_board: Board,
    limits: SearchLimits,

    info: Info,
}

impl Logger for Search {}

impl Search {
    /// Creates a new `Search` instance with the given board and evaluator.
    ///
    /// # Arguments
    ///
    /// * `board` - The current board state.
    /// * `evaluator` - The evaluator used to evaluate the board.
    /// * `limits` - The search limits.
    ///
    /// # Returns
    ///
    /// * `Search` - A new `Search` instance.
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let search = Search::new(&board, None);
    /// ```
    pub fn new(board: &Board, limits: Option<SearchLimits>) -> Self {
        Self {
            board: board.clone(),
            original_board: board.clone(),
            limits: limits.unwrap_or_default(),
            running: Arc::new(AtomicBool::new(true)),

            info: Info::new(),
        }
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
    /// ```
    pub fn search(&mut self, evaluator: &impl Evaluator, max_depth: Option<Depth>) {
        // Uses a heuristic to determine the maximum time to spend on a move
        self.start();

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

        self.iter_deep(evaluator, max_depth);

        self.stop();
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
    fn iter_deep(&mut self, evaluator: &impl Evaluator, max_depth: Option<Depth>) {
        let start = Instant::now();
        for depth in 1..=max_depth.unwrap_or(Depth::MAX) {
            self.alpha_beta_start(evaluator, depth, start);

            if !self.is_running() || self.limits_exceeded(start) {
                break;
            }

            let pv = self.get_pv(depth);
            self.log_uci_info(
                depth,
                self.info.nodes,
                start.elapsed().as_millis(),
                self.info.best_score.unwrap_or(0),
                &pv,
            );
        }

        self.log(format!("bestmove {}", self.info.best_move.unwrap()).as_str());
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
    fn alpha_beta_start(
        &mut self,
        evaluator: &impl Evaluator,
        depth: Depth,
        start: Instant,
    ) -> Ply {
        let moves = self.board.get_legal_moves();
        if moves.is_empty() {
            return Ply::default();
        }

        let mut alpha = i64::MIN;
        let beta = i64::MAX;
        let mut best_ply = moves[0];
        for mv in MoveOrderer::new(&moves, self.board.zkey) {
            self.board.make_move(mv);
            self.info.nodes += 1;

            let score = self
                .alpha_beta(
                    evaluator,
                    beta.saturating_neg(),
                    alpha.saturating_neg(),
                    depth - 1,
                    start,
                )
                .saturating_neg();

            if !self.is_running() || self.limits_exceeded(start) {
                return best_ply;
            }

            if score > alpha {
                alpha = score;
                best_ply = mv;
            }
            self.board.unmake_move();
        }

        // Don't save incomplete searches
        if self.is_running() && !self.limits_exceeded(start) {
            TRANSPOSITION_TABLE
                .write()
                .expect("Transposition table is poisoned! Unable to write new entry.")
                .insert(
                    self.board.zkey,
                    TTEntry {
                        score: alpha,
                        depth,
                        bound: Bounds::Exact,
                        best_ply,
                    },
                );

            self.info.best_score = Some(alpha);
            self.info.best_move = Some(best_ply);
        }

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
    fn alpha_beta(
        &mut self,
        evaluator: &impl Evaluator,
        alpha_start: i64,
        beta_start: i64,
        depth: Depth,
        start: Instant,
    ) -> i64 {
        if !self.is_running() || self.limits_exceeded(start) {
            return 0;
        }

        if self.board.get_halfmove_clock() >= 100 {
            return 0; // Draw by fifty-move rule
        }

        if self.board.position_reached(self.board.zkey) {
            return 0; // Avoid threefold repetition at first repeitition
        }

        if depth == 0 {
            return evaluator.evaluate(&mut self.board);
        }

        let mut alpha = alpha_start;
        let mut beta = beta_start;

        // Check if we have more information in the TTable than we have already reached in this search
        if let Some(entry) = TRANSPOSITION_TABLE
            .read()
            .expect("Transposition table is poisoned! Unable to read entry.")
            .get(&self.board.zkey)
        {
            if entry.depth >= depth {
                match entry.bound {
                    Bounds::Exact => return entry.score,
                    Bounds::Lower => alpha = alpha.max(entry.score),
                    Bounds::Upper => beta = beta.min(entry.score),
                }

                if alpha >= beta {
                    return entry.score;
                }
            }
        }

        let moves = self.board.get_legal_moves();
        if moves.is_empty() {
            if self.board.is_in_check(self.board.current_turn) {
                return i64::MIN; // Checkmate
            }
            return 0; // Stalemate
        }

        let mut best_ply = moves[0];
        for mv in MoveOrderer::new(&moves, self.board.zkey) {
            self.board.make_move(mv);
            self.info.nodes += 1;
            let score = self
                .alpha_beta(
                    evaluator,
                    beta.saturating_neg(),
                    alpha.saturating_neg(),
                    depth - 1,
                    start,
                )
                .saturating_neg();
            self.board.unmake_move();

            // Move is too good, opponent will not allow the game to reach this position
            if score >= beta {
                TRANSPOSITION_TABLE
                    .write()
                    .expect("Transposition table is poisoned! Unable to write new entry.")
                    .insert(
                        self.board.zkey,
                        TTEntry {
                            score,
                            depth,
                            bound: Bounds::Lower,
                            best_ply: mv,
                        },
                    );
                return beta;
            }

            // New best move
            if score > alpha {
                alpha = score;
                best_ply = mv;
            }
        }

        TRANSPOSITION_TABLE
            .write()
            .expect("Transposition table is poisoned! Unable to write new entry.")
            .insert(
                self.board.zkey,
                TTEntry {
                    score: alpha,
                    depth,
                    bound: if alpha <= alpha_start {
                        Bounds::Upper
                    } else {
                        Bounds::Exact
                    },
                    best_ply,
                },
            );

        alpha
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
    fn limits_exceeded(&self, start: Instant) -> bool {
        if let Some(nodes) = self.limits.nodes {
            if self.info.nodes >= nodes {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }

        let duration = start.elapsed();
        let time_elapsed_in_ms = duration.as_millis();
        time_elapsed_in_ms >= self.limits.movetime.unwrap_or(Millisecond::MAX)
            || ([
                self.limits.white_time,
                self.limits.white_increment,
                self.limits.black_time,
                self.limits.black_increment,
            ]
            .iter()
            .any(Option::is_some)
                && time_elapsed_in_ms
                    >= self
                        .limits
                        .time_management_timer
                        .unwrap_or(Millisecond::MAX))
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
    #[allow(clippy::cast_possible_truncation)]
    fn log_uci_info(
        &self,
        depth: Depth,
        nodes: NodeCount,
        time_elapsed_in_ms: Millisecond,
        best_value: i64,
        pv: &[Ply],
    ) {
        let score = match best_value {
            i64::MIN | NEGMAX => format!("mate -{}", pv.len().div_ceil(2)),
            i64::MAX => format!("mate {}", pv.len().div_ceil(2)),
            _ => format!("cp {best_value}"),
        };
        let nps: u64 = nodes / (time_elapsed_in_ms as u64 / 1000).max(1);
        let pv_notation: Vec<String> = pv.iter().map(std::string::ToString::to_string).collect();
        let pv_string = pv_notation.join(" ");
        self.log(
            format!("info depth {depth} nodes {nodes} time {time_elapsed_in_ms} nps {nps} score {score} pv {pv_string}")
                .as_str(),
        );
    }

    /// Returns the principal variation (PV) of the search.
    /// The principal variation is the best line of play found by the search.
    /// The PV is generated out to the maximum length sepcified, or earlier if the transposition table does not hold any more records
    ///
    /// # Arguments
    ///
    /// * `length` - The maximum length of the PV to return.
    ///
    /// # Returns
    ///
    /// * `Vec<Ply>` - A vector of `Ply` representing the best line of play found by the search.
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.search(Some(3));
    /// let pv = search.get_pv(3);
    /// assert_eq!(pv.len(), 3);
    /// ```
    fn get_pv(&mut self, length: Depth) -> Vec<Ply> {
        let mut plys = Vec::new();
        let tt = TRANSPOSITION_TABLE
            .read()
            .expect("Transposition table is poisoned! Unable to read entry.");
        let original_zkey = self.original_board.zkey;

        for _ in 0..length {
            if let Some(entry) = tt.get(&self.original_board.zkey) {
                plys.push(entry.best_ply);
                self.original_board.make_move(entry.best_ply);
            } else {
                break;
            }
        }

        for _ in &plys {
            self.original_board.unmake_move();
        }

        assert!(
            self.original_board.zkey == original_zkey,
            "The original board has been modified!",
        );

        plys
    }

    /// Returns the number of nodes searched.
    ///
    /// # Returns
    ///
    /// * `u64` - The number of nodes searched.
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let nodes = search.get_nodes();
    /// ```
    pub const fn get_nodes(&self) -> NodeCount {
        self.info.nodes
    }

    /// Sets the `AtomicBool` that is used to determine if the search should continue to true
    /// Normally called by the search function.
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.stop();
    /// assert_eq!(search.is_running(), false);
    /// search.start();
    /// assert_eq!(search.is_running(), true);
    /// ```
    fn start(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    /// Sets the `AtomicBool` that is used to determine if the search should continue to false
    /// The search will wrap up as soon as possible and stop.
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// search.stop();
    /// assert_eq!(search.is_running(), false);
    /// ```
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
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
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
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
    fn test_log_uci() {
        let board = BoardBuilder::construct_starting_board().build();
        let search = Search::new(&board, None);
        search.log_uci_info(3, 20000, 1500, 10, &[Ply::default()]);
    }

    #[test]
    fn test_check_limits() {
        let board = BoardBuilder::construct_starting_board().build();
        let start = Instant::now();
        let mut search = Search::new(&board, None);
        assert!(!search.limits_exceeded(start));
        search.limits.nodes = Some(100);
        assert!(!search.limits_exceeded(start));
        search.info.nodes = 100;
        assert!(search.limits_exceeded(start));
        search.limits.nodes = None;
        search.limits.movetime = Some(1000);
        assert!(!search.limits_exceeded(start));
    }

    #[test]
    fn test_search_no_legal_moves() {
        let board = Board::from_fen("4r2k/p2q1RQb/1p5p/2ppP3/3P4/2P5/P1P1B1PP/5RK1 b - - 0 22");
        let mut search = Search::new(&board, None);
        let best_move = search.alpha_beta_start(&SimpleEvaluator, 5, Instant::now());
        assert_eq!(best_move, Ply::default());
    }

    #[test]
    fn test_alpha_beta() {
        let board = BoardBuilder::construct_starting_board().build();
        let mut search = Search::new(&board, None);
        let score = search.alpha_beta(&SimpleEvaluator, i64::MIN, i64::MAX, 4, Instant::now());
        assert_eq!(score, 0)
    }

    #[bench]
    fn bench_search_depth_3(bencher: &mut Bencher) {
        bencher.iter(|| {
            Search::new(&BoardBuilder::construct_starting_board().build(), None)
                .search(&SimpleEvaluator, Some(3));
            TRANSPOSITION_TABLE
                .write()
                .expect("Transposition table is poisoned! Unable to write new entry.")
                .clear();
        });
    }

    #[bench]
    fn bench_search_depth_4(bencher: &mut Bencher) {
        bencher.iter(|| {
            Search::new(&BoardBuilder::construct_starting_board().build(), None)
                .search(&SimpleEvaluator, Some(4));
            TRANSPOSITION_TABLE
                .write()
                .expect("Transposition table is poisoned! Unable to write new entry.")
                .clear();
        });
    }

    #[bench]
    fn bench_search_depth_5(bencher: &mut Bencher) {
        bencher.iter(|| {
            Search::new(&BoardBuilder::construct_starting_board().build(), None)
                .search(&SimpleEvaluator, Some(5));
            TRANSPOSITION_TABLE
                .write()
                .expect("Transposition table is poisoned! Unable to write new entry.")
                .clear();
        });
    }

    #[bench]
    fn bench_search_depth_6(bencher: &mut Bencher) {
        bencher.iter(|| {
            Search::new(&BoardBuilder::construct_starting_board().build(), None)
                .search(&SimpleEvaluator, Some(6));
            TRANSPOSITION_TABLE
                .write()
                .expect("Transposition table is poisoned! Unable to write new entry.")
                .clear();
        });
    }
}
