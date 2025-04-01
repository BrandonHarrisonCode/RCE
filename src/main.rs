#![feature(test)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::decimal_literal_representation,
    clippy::format_push_string
)]

#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod bench;
mod board;
mod evaluate;
mod search;
mod uci;
mod utils;

fn main() {
    board::zkey::ZTable::init();
    uci::start();
}
