use crate::board::transposition_table::TRANSPOSITION_TABLE;
use crate::board::zkey::ZKey;
use crate::board::Ply;

mod scored_ply;

use scored_ply::ScoredPly;

#[non_exhaustive]
struct ScoreBonus;

impl ScoreBonus {
    pub const CAPTURE: MoveScore = 4000;
    pub const PROMOTION: MoveScore = 3000;
}

type MoveScore = u64;
pub struct MoveOrderer {
    scored_moves: Vec<ScoredPly>,
    index: usize,
}

fn score_moves(zkey: &ZKey, moves: &[Ply]) -> Vec<ScoredPly> {
    let best_ply = TRANSPOSITION_TABLE
        .read()
        .expect("Transposition table is poisoned! Unable to read entry.")
        .get(zkey)
        .map(|entry| entry.best_ply);

    moves
        .iter()
        .map(|&ply| {
            let score = score_move(ply, best_ply);
            ScoredPly { ply, score }
        })
        .collect()
}

fn score_move(ply: Ply, best_ply: Option<Ply>) -> MoveScore {
    let mut score: MoveScore = 0;
    if best_ply.is_some_and(|best_ply| best_ply == ply) {
        return MoveScore::MAX;
    }
    if ply.is_capture() {
        score += ScoreBonus::CAPTURE;
    }
    if ply.is_promotion() {
        score += ScoreBonus::PROMOTION;
    }
    score
}

impl MoveOrderer {
    pub fn new(moves: &[Ply], zkey: ZKey) -> Self {
        let scored_moves = score_moves(&zkey, &moves);

        Self {
            scored_moves,
            index: 0,
        }
    }
}

impl Iterator for MoveOrderer {
    type Item = Ply;

    /// Use selection sort instead of a faster sort because most entries will be beyond the cuttoff and will never be examined
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.scored_moves.len() {
            return None;
        }

        let mut best_index: usize = self.index;
        let mut best_score = self.scored_moves[self.index].score;

        for i in (self.index + 1)..self.scored_moves.len() {
            let score = self.scored_moves[i].score;
            if score > best_score {
                best_index = i;
                best_score = score;
            }
        }

        self.scored_moves.swap(self.index, best_index);
        self.index += 1;

        Some(self.scored_moves[self.index - 1].ply)
    }
}
