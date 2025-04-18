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
pub type Score = i16;
pub type NodeCount = u64;
pub type Millisecond = u128;

const NEGMAX: Score = -Score::MAX; // Score::MIN + 1

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
        let moves = self.board.get_all_moves();
        if moves.is_empty() {
            return Ply::default();
        }

        let mut total_legal_moves = 0;
        let mut alpha = i16::MIN;
        let beta = i16::MAX;
        let mut best_ply = moves[0];
        let mut pvs = false;
        let killers = self.info.killers[usize::from(self.info.depth)];
        for mv in MoveOrderer::new(&moves, self.board.zkey, &killers) {
            if self.board.is_legal_move(mv).is_err() {
                continue; // Skip illegal moves
            }
            total_legal_moves += 1;

            self.board.make_move(mv);
            self.info.nodes += 1;

            let mut score;
            self.info.depth += 1;
            if pvs {
                score = self
                    .alpha_beta(
                        evaluator,
                        alpha.saturating_neg() - 1,
                        alpha.saturating_neg(),
                        depth - 1,
                        start,
                    )
                    .saturating_neg();
                // Principal variation search failed, the score is different then our bounds
                if alpha < score && score < beta {
                    score = self
                        .alpha_beta(
                            evaluator,
                            beta.saturating_neg(),
                            alpha.saturating_neg(),
                            depth - 1,
                            start,
                        )
                        .saturating_neg();
                }
            } else {
                score = self
                    .alpha_beta(
                        evaluator,
                        beta.saturating_neg(),
                        alpha.saturating_neg(),
                        depth - 1,
                        start,
                    )
                    .saturating_neg();
            }
            self.info.depth -= 1;

            if !self.is_running() || self.limits_exceeded(start) {
                // Don't throw out a partial search just because the current move was not searched
                if self.info.best_score.is_some_and(|s| alpha > s) {
                    self.info.best_score = Some(alpha);
                    self.info.best_move = Some(best_ply);
                }
                return best_ply;
            }

            if score > alpha {
                alpha = score;
                best_ply = mv;
                pvs = true;
            }
            self.board.unmake_move();
        }

        if total_legal_moves == 0 {
            return Ply::default();
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
    /// * `Score` - The score of the "best" position
    ///
    /// # Example
    /// ```
    /// let board = BoardBuilder::construct_starting_board().build();
    /// let evaluator = SimpleEvaluator::new();
    /// let mut search = Search::new(&board, &evaluator, None);
    /// let score = search.alpha_beta(Score::MIN, Score::MAX, 3);
    /// ```
    #[allow(clippy::too_many_lines)]
    fn alpha_beta(
        &mut self,
        evaluator: &impl Evaluator,
        alpha_start: Score,
        beta_start: Score,
        mut depth: Depth,
        start: Instant,
    ) -> Score {
        if !self.is_running() || self.limits_exceeded(start) {
            return 0;
        }

        let mut alpha = alpha_start;
        let mut beta = beta_start;

        if self.board.get_halfmove_clock() >= 100 {
            return 0; // Draw by fifty-move rule
        }

        if self.board.position_reached(self.board.zkey) {
            return 0; // Avoid threefold repetition at first repeitition
        }

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

        // Get out of check before entering quiescence
        if self.board.is_in_check(self.board.current_turn) {
            // Since this is called from alpha_beta_start, depth will always be at least (Depth::MAX - 1).
            // However, if we add more extensions, we need to be concerned with overflow in the future.
            depth += 1;
        }

        if depth == 0 {
            return self.quiescence(evaluator, alpha, beta, start);
        }

        let moves = self.board.get_all_moves();
        let mut total_legal_moves = 0;

        let mut best_ply = moves[0];
        let mut pvs = false;
        let killers = self.info.killers[usize::from(self.info.depth)];
        for mv in MoveOrderer::new(&moves, self.board.zkey, &killers) {
            if self.board.is_legal_move(mv).is_err() {
                continue; // Skip illegal moves
            }
            total_legal_moves += 1;

            self.board.make_move(mv);
            self.info.nodes += 1;

            let mut score;
            self.info.depth += 1;
            if pvs {
                score = self
                    .alpha_beta(
                        evaluator,
                        alpha.saturating_neg() - 1,
                        alpha.saturating_neg(),
                        depth - 1,
                        start,
                    )
                    .saturating_neg();
                // Principal variation search failed, the score is different then our bounds
                if alpha < score && score < beta {
                    score = self
                        .alpha_beta(
                            evaluator,
                            beta.saturating_neg(),
                            alpha.saturating_neg(),
                            depth - 1,
                            start,
                        )
                        .saturating_neg();
                }
            } else {
                score = self
                    .alpha_beta(
                        evaluator,
                        beta.saturating_neg(),
                        alpha.saturating_neg(),
                        depth - 1,
                        start,
                    )
                    .saturating_neg();
            }
            self.info.depth -= 1;

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

                self.store_killers(mv);

                return beta;
            }

            // New best move
            if score > alpha {
                alpha = score;
                best_ply = mv;
                pvs = true;
            }
        }

        if total_legal_moves == 0 {
            if self.board.is_in_check(self.board.current_turn) {
                return Score::MIN + i16::from(self.info.depth); // Checkmate (with tiebreaker being shortest depth)
            }
            return 0; // Stalemate
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

    fn quiescence(
        &mut self,
        evaluator: &impl Evaluator,
        alpha_start: Score,
        beta_start: Score,
        start: Instant,
    ) -> Score {
        if !self.is_running() || self.limits_exceeded(start) {
            return 0;
        }

        let mut alpha = alpha_start;
        let beta = beta_start;

        let score = evaluator.evaluate(&mut self.board);
        if score >= beta {
            return beta; // No need to search any further, beta cutoff
        }

        if score > alpha {
            alpha = score; // New best move
        }

        // We don't check for fifty move rule or three-fold because every move in quiescence is a capture

        let moves: Vec<Ply> = self.board.get_filtered_moves(Ply::is_capture);

        let killers = self.info.killers[usize::from(self.info.depth)];
        for mv in MoveOrderer::new(&moves, self.board.zkey, &killers) {
            if self.board.is_legal_move(mv).is_err() {
                continue; // Skip illegal moves
            }

            self.board.make_move(mv);
            self.info.nodes += 1;

            self.info.depth += 1;
            let score = self
                .quiescence(
                    evaluator,
                    beta.saturating_neg(),
                    alpha.saturating_neg(),
                    start,
                )
                .saturating_neg();
            self.info.depth -= 1;

            self.board.unmake_move();

            // Move is too good, opponent will not allow the game to reach this position
            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

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
        if self.info.depth == Depth::MAX {
            return true;
        }
        if let Some(nodes) = self.limits.nodes {
            if self.info.nodes >= nodes {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }
        if let Some(depth) = self.limits.depth {
            if self.info.depth >= depth {
                self.running.store(false, Ordering::Relaxed);
                return true;
            }
        }
        if let Some(movetime) = self.limits.movetime {
            if start.elapsed().as_millis() >= movetime {
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
        best_value: Score,
        pv: &[Ply],
    ) {
        let score = match best_value {
            Score::MIN | NEGMAX => format!("mate -{}", pv.len().div_ceil(2)),
            Score::MAX => format!("mate {}", pv.len().div_ceil(2)),
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

    /// Stores the killer moves for the current depth
    ///
    /// # Arguments
    ///
    /// * `ply` - The move to store as a killer move
    fn store_killers(&mut self, ply: Ply) {
        if ply.is_capture() || ply.is_promotion() {
            return; // A killer move is a quiet move, we're already considering captures and promotions first
        }

        if self.info.killers[usize::from(self.info.depth)][0] != Some(ply) {
            self.info.killers[usize::from(self.info.depth)][1] =
                self.info.killers[usize::from(self.info.depth)][0];
            self.info.killers[usize::from(self.info.depth)][0] = Some(ply);
        }
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
                let best_ply = entry.best_ply;
                if self.original_board.is_legal_move(best_ply).is_err() {
                    break;
                }
                plys.push(best_ply);
                self.original_board.make_move(best_ply);
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

    /// Designed to catch bug where we access the killer table set on a previous search out of bounds because we're searching to max depth
    #[test]
    fn test_mate_in_2() {
        let mut board = Board::from_fen("8/1R6/2N2P2/2kP4/2P4P/3P4/8/6K1 w - - 1 94");
        let mut search = Search::new(&board, None);
        search.search(&SimpleEvaluator, Some(6));
        let search_move = search
            .info
            .best_move
            .expect("No best move found during search!");
        let best_move = board
            .find_move("f6f7")
            .expect("Move not found in legal moves!");

        assert_eq!(search_move, best_move);
        board.make_move(best_move);

        let best_move = board
            .find_move("c5d6")
            .expect("Move not found in legal moves!");
        board.make_move(best_move);

        let best_move_queen = board
            .find_move("f7f8q")
            .expect("Move not found in legal moves!");

        let best_move_bishop = board
            .find_move("f7f8b")
            .expect("Move not found in legal moves!");

        search = Search::new(&board, None);
        search.search(&SimpleEvaluator, Some(Depth::MAX));
        let search_move = search
            .info
            .best_move
            .expect("No best move found during search!");
        assert!(
            search_move == best_move_queen || search_move == best_move_bishop,
            "Search found {search_move} was the best move, but the best move was {best_move_queen} or {best_move_bishop}"
        );
    }

    #[test]
    fn test_alpha_beta() {
        let board = BoardBuilder::construct_starting_board().build();
        let mut search = Search::new(&board, None);
        let score = search.alpha_beta(&SimpleEvaluator, Score::MIN, Score::MAX, 4, Instant::now());
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
