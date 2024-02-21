#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod utils;

use board::Board;

const TITLE: &str = "Rust Chess Engine";
const SHORT_TITLE: &str = "RCE";

fn main() {
    println!("{TITLE} - {SHORT_TITLE}");
    let mut board = Board::construct_starting_board();

    println!("{board}");

    let moves = board.get_legal_moves();

    for pmove in moves {
        board.make_move(pmove);
        println!("{pmove}:\n{board}");
        // println!("Iterating moves...");
        // let new_moves = board.get_all_moves();
        // for npmove in new_moves {
        //     dbg!(npmove);
        //     println!("Before:\n{}", board);
        //     board.make_move(npmove);
        //     println!("After:\n{}", board);
        //     board.unmake_move(npmove);
        // }
        // println!("Ending iteration...");
        board.unmake_move();
    }
}
