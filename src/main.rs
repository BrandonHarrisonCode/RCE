#[macro_use]
extern crate strum_macros;

mod board;

fn main() {
    let board = board::create_starting_board();

    println!("{}", board);
}
