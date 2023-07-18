#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;

fn main() {
    println!("Rust Chess Engine - RCE");
    let board = board::create_starting_board();

    println!("{}", board);
    println!("{:?}", board.get_moves_for_piece_at_coords(1, 0).unwrap());
}
