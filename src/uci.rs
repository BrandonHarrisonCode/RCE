use crate::board::{Board, BoardBuilder};

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

pub fn start() {
    let mut board = BoardBuilder::construct_starting_board();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        let fields: Vec<&str> = trimmed.split_whitespace().collect();
        let token = fields[0];

        match token {
            "uci" => print_engine_info(),
            "isready" => println!("readyok"),
            "ucinewgame" => board = BoardBuilder::construct_starting_board(),
            "position" => {
                if fields[1] == "startpos" {
                    board = BoardBuilder::construct_starting_board();
                }
                if fields[1] == "fen" {
                    board = Board::from_fen(fields[2]);
                } else {
                    println!("Not supported");
                }
                if fields.len() > 2 && fields[2] == "moves" {
                    for mv in fields.iter().skip(3) {
                        println!("Playing moves on position string not supported");
                    }
                }
            }
            "go" => println!("Not supported"),
            "stop" => println!("Not supported"),
            "quit" => break,
            "setoption" => println!("Not supported"),
            "debug" => println!("Not supported"),
            _ => println!("Invalid command!"),
        }
    }
}

fn print_engine_info() {
    println!("id name {TITLE}");
    println!("id author {AUTHOR}");
    println!("uciok");
}
