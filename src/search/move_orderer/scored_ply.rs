use crate::board::Ply;

pub struct ScoredPly {
    pub ply: Ply,
    pub score: u64,
}
