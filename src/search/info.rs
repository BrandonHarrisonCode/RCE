use crate::board::Ply;

use super::{Depth, Score};

pub const MAX_KILLERS: usize = 2;
pub type KillerList = [[Option<Ply>; MAX_KILLERS]; Depth::MAX as usize];

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
    pub best_score: Option<Score>,
    pub nodes: u64,
    pub depth: Depth,
    pub killers: KillerList,
}

impl Info {
    /// Creates a new `Info` instance with default values.
    pub const fn new() -> Self {
        Self {
            best_move: None,
            best_score: None,
            nodes: 0,
            depth: 0,
            killers: [[None; MAX_KILLERS]; Depth::MAX as usize],
        }
    }
}
