#![feature(test)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod evaluate;
mod search;
mod utils;

use board::piece::Color;
use board::BoardBuilder;
use board::Ply;
use evaluate::simple_evaluator::SimpleEvaluator;

const TITLE: &str = "Rust Chess Engine";
const SHORT_TITLE: &str = "RCE";

fn main() {
    println!("{TITLE} - {SHORT_TITLE}");
    let evaluator = SimpleEvaluator::new();

    let mut board = BoardBuilder::construct_starting_board();

    println!("{board}");

    loop {
        if board.is_game_over() {
            println!("Game over! {:#?}", board.game_state);
            break;
        }

        if board.current_turn == Color::White {
            let mut line = String::new();
            println!("Enter your move (eg. b2b4): ");
            std::io::stdin().read_line(&mut line).unwrap();

            let player_move = Ply::parse_move(line.trim());
            let moves = board.get_legal_moves();
            let filtered_move = moves
                .iter()
                .find(|mv| mv.start == player_move.start && mv.dest == player_move.dest)
                .expect("Invalid move");
            board.make_move(*filtered_move);

            println!("{filtered_move}:\n{board}");
        } else {
            let computer_move = search::search(&mut board, &evaluator);
            println!("Computer's move: {computer_move}");
            board.make_move(computer_move);
            println!("{computer_move}:\n{board}");
        }
        println!();
    }
}
