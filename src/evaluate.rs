use super::board::Board;

pub mod simple_evaluator;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i32;
}
