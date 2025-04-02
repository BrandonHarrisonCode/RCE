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
mod testing_utils;
mod uci;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "bench" {
            bench::bench();
            return;
        }
    }

    board::zkey::ZTable::init();
    uci::start();
}
