use std::thread;

use crate::board::{Board, BoardBuilder};

use crate::evaluate::simple_evaluator::SimpleEvaluator;
use crate::search::limits::SearchLimits;
use crate::search::Search;

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

const EVALUATOR: SimpleEvaluator = SimpleEvaluator::new();

pub fn start() {
    let mut board = BoardBuilder::construct_starting_board();

    loop {
        println!("{board}");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        let fields: Vec<&str> = trimmed.split_whitespace().collect();

        if fields.is_empty() {
            continue;
        }
        let token = fields[0];

        #[allow(clippy::match_same_arms)]
        match token {
            "uci" => print_engine_info(),
            "isready" => println!("readyok"),
            "ucinewgame" => board = BoardBuilder::construct_starting_board(),
            "position" => {
                board = load_position(&fields)
                    .inspect_err(|e| eprintln!("Failed to set position: {e}"))
                    .unwrap_or(board);
            }
            "go" => {
                let _ = go(&board, &fields)
                    .inspect_err(|e| eprintln!("Failed to execute go command: {e}"));
            }
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

fn load_position(fields: &[&str]) -> Result<Board, String> {
    let mut board = BoardBuilder::construct_starting_board();

    if fields.len() < 2 {
        return Err("No position specified!".to_string());
    }

    match fields[1] {
        "startpos" => {}
        "fen" => {
            if fields.len() < 3 {
                return Err("No FEN specified!".to_string());
            }
            board = Board::from_fen(fields[2..].join(" ").as_str());
        }
        _ => return Err(format!("Unrecognized position command: {}", fields[1])),
    }

    if fields.len() > 2 && fields[2] == "moves" {
        return Err("Specifying moves is currently not suppoerted! Try again.".to_string());
    }

    Ok(board)
}

fn go(board: &Board, fields: &[&str]) -> Result<(), String> {
    let mut limits = SearchLimits::new();

    let mut idx = 1;
    while idx < fields.len() {
        let token = fields[idx];

        match token {
            "searchmoves" => return Err("searchmoves is not supported!".to_string()),
            "ponder" => return Err("ponder is not supported!".to_string()),
            "wtime" => {
                idx += 1;
                limits = limits.white_time(parse_value(fields[idx], token));
            }
            "btime" => {
                idx += 1;
                limits = limits.black_time(parse_value(fields[idx], token));
            }
            "winc" => {
                idx += 1;
                limits = limits.white_increment(parse_value(fields[idx], token));
            }
            "binc" => {
                idx += 1;
                limits = limits.black_increment(parse_value(fields[idx], token));
            }
            "movestogo" => return Err("movestogo is not supported!".to_string()),
            "depth" => {
                idx += 1;
                limits = limits.depth(parse_value(fields[idx], token));
            }
            "nodes" => {
                idx += 1;
                limits = limits.nodes(parse_value(fields[idx], token));
            }
            "mate" => return Err("mate is not supported!".to_string()),
            "movetime" => {
                idx += 1;
                limits = limits.movetime(parse_value(fields[idx], token));
            }
            "infinite" => {
                limits = limits.depth(None);
            }
            _ => return Err("Invalid go command!".to_string()),
        };

        idx += 1;
    }

    let mut search = Search::new(board, &EVALUATOR, Some(limits));
    thread::spawn(move || {
        println!("detatching search thread");
        let best_move = search.search(None);
        println!("bestmove {best_move}");
    });

    Ok(())
}

fn parse_value<T>(str: &str, kind: &str) -> Option<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let result = str
        .parse()
        .map_err(|e| format!("Failed to parse value \"{e}\" for {kind}!"));
    if let Err(err_str) = result {
        eprintln!("{err_str}");
        return None;
    }

    Some(result.unwrap())
}
