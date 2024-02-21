#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod utils;

use board::piece::Color;
use board::Board;
use board::Ply;

use rand::seq::SliceRandom;
use rand::thread_rng;

const TITLE: &str = "Rust Chess Engine";
const SHORT_TITLE: &str = "RCE";

fn main() {
    println!("{TITLE} - {SHORT_TITLE}");
    let mut board = Board::construct_starting_board();

    println!("{board}");
    let mut rng = thread_rng();

    loop {
        let moves = board.get_legal_moves();
        if moves.is_empty() {
            println!("Game over!");
            break;
        }

        if board.current_turn == Color::White {
            let mut line = String::new();
            println!("Enter your move (eg. b2b4): ");
            std::io::stdin().read_line(&mut line).unwrap();

            let player_move = Ply::parse_move(line.trim());
            let filtered_move = moves
                .iter()
                .find(|mv| mv.start == player_move.start && mv.dest == player_move.dest)
                .expect("Invalid move");
            board.make_move(*filtered_move);

            println!("{filtered_move}:\n{board}");
        } else {
            let computer_move = moves.choose(&mut rng).unwrap();
            println!("Computer's move: {computer_move}");
            board.make_move(*computer_move);
            println!("{computer_move}:\n{board}");
        }
        println!();
    }
}
