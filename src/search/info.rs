use crate::board::Ply;

/// Information about the search process.
/// Usually displayed in the uci `info` log.
///
/// # Fields
///
/// - `best_move`: The best move found so far.
/// - `depth`: The current search depth.
/// - `nodes`: The number of nodes searched.
/// - `movetime`: The time taken for the search.
///
/// # Example
///
/// ```
/// use crate::search::info::Info;
/// use crate::search::Ply;
/// use crate::search::Depth;
///
/// let info = Info {
///   best_move: Some(Ply::default()),
///   depth: Depth::new(5),
///   nodes: 1000,
///   movetime: 200,
/// };
///
/// println!("Best move: {:?}", info.best_move);
/// ```
pub struct Info {
    pub best_move: Option<Ply>,
    pub best_score: Option<i64>,
    pub nodes: u64,
}

impl Info {
    /// Creates a new `Info` instance with default values.
    pub const fn new() -> Self {
        Self {
            best_move: None,
            best_score: None,
            nodes: 0,
        }
    }
}
