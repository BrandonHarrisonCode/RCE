#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod utils;

const TITLE: &str = "Rust Chess Engine";
const SHORT_TITLE: &str = "RCE";

fn main() {
    println!("{TITLE} - {SHORT_TITLE}");
    let bb = 0b0000000000000000000000000000000000000000000000001111111100000000;
    utils::debug_bitboard(&bb);

    let mut board = board::create_starting_board();

    println!("{}", board);

    let moves = board.get_all_moves();

    println!("{:?}", moves);

    for pmove in moves {
        board.make_move(pmove);
        println!("{}", board);
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
        board.unmake_move(pmove);
    }
}
