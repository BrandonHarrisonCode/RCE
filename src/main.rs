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

    println!("{}", board);

    let moves = board.get_all_moves();

    println!("{:?}", moves);

    let test_moves = board
        .get_moves_for_piece(&board::piece::Square::new(1, 0))
        .unwrap();
    board.make_move(test_moves[0]);
    println!("{}", board);
}
