#[macro_use]
extern crate strum_macros;

mod board;

fn main() {
    println!("Rust Chess Engine - RCE");
    let board = board::create_starting_board();

    println!("{}", board);
}
