#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod utils;

const TITLE: &str = "Rust Chess Engine";
const SHORT_TITLE: &str = "RCE";

fn main() {
    println!("{TITLE} - {SHORT_TITLE}");

    let mut board = board::create_starting_board();

    let moves = board
        .get_moves_for_piece(&board::piece::Square::new(1, 0))
        .unwrap();
    println!("{}", board);
    println!("{:?}", moves);

    board.make_move(moves[0]);
    println!("{}", board);
}
