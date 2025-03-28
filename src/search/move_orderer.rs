mod ScoreBonus;
mod ScoredPly;
use crate::board::{Board, Ply};

pub struct MoveOrderer {
    pub board: Board,
    pub moves: Vec<Ply>,
    scored_moves: Vec<ScoredPly>,
    index: usize,
}

impl MoveOrderer {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            moves: board.get_legal_moves(),
            index: 0,
        }
    }

    pub fn get_moves(&self) -> Vec<Ply> {
        self.moves.clone()
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    fn score_moves(&mut self) {
        self.scored_moves = self
            .moves
            .iter()
            .map(|&ply| {
                let score = self.score_move(ply);
                ScoredPly { ply, score }
            })
            .collect();
    }

    fn score_move(&self, ply: Ply) -> u64 {
        let mut score = 0;
        if self.board.is_check(ply) {
            score += ScoreBonus::Check;
        }
        if self.board.is_capture(ply) {
            score += ScoreBonus::Capture;
        }
        if self.board.is_promotion(ply) {
            score += ScoreBonus::Promotion;
        }
        score
    }
}

impl Iterator for MoveOrderer {
    type Item = Ply;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moves.is_empty() {
            None
        } else {
            Some(self.moves.remove(0))
        }
    }
}
