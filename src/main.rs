#![feature(test)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[macro_use]
extern crate strum_macros;
extern crate derive_more;

mod board;
mod evaluate;
mod search;
mod uci;
mod utils;

fn main() {
    uci::start();
}
