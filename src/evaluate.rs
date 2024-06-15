use super::board::Board;

pub mod simple_evaluator;

pub trait Evaluator: Clone {
    fn evaluate(&self, board: &Board) -> i64;
}
