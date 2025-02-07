use build_time::build_time_utc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::board::{Board, BoardBuilder};

use crate::evaluate::simple_evaluator::SimpleEvaluator;
use crate::search::limits::SearchLimits;
use crate::search::Search;

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

const VERSION: &str = build_time_utc!("%Y.%m.%d %H:%M:%S");

pub fn start() {
    let mut board = BoardBuilder::construct_starting_board().build();
    let mut search_running: Option<Arc<AtomicBool>> = None;
    let mut join_handle: Option<thread::JoinHandle<()>> = None;

    loop {
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
            "ucinewgame" => board = BoardBuilder::construct_starting_board().build(),
            "position" => {
                board = load_position(&fields)
                    .inspect_err(|e| eprintln!("Failed to set position: {e}"))
                    .unwrap_or(board);
            }
            "go" => {
                if let Some(jh) = &join_handle {
                    if !jh.is_finished() {
                        eprintln!("Search is already running!");
                        continue;
                    }
                }
                if let Ok((new_search, new_join_handle)) = go(&board, &fields) {
                    search_running = Some(new_search);
                    join_handle = Some(new_join_handle);
                } else {
                    eprintln!("Failed to execute go command!");
                }
            }
            "stop" => {
                if let Some(is_running) = &search_running {
                    is_running.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            }
            "quit" => break,
            "setoption" => println!("Not supported"),
            "debug" => println!("Not supported"),
            _ => println!("Invalid command!"),
        }
    }
}

fn print_engine_info() {
    println!("id name {TITLE} {VERSION}");
    println!("id author {AUTHOR}");
    println!("uciok");
}

fn load_position(fields: &[&str]) -> Result<Board, String> {
    let mut board = BoardBuilder::construct_starting_board().build();
    let mut idx = 1;

    if fields.len() < 2 {
        return Err("No position specified!".to_string());
    }

    match fields[idx] {
        "startpos" => idx += 1,
        "fen" => {
            if fields.len() < 8 {
                return Err("No FEN specified!".to_string());
            }
            board = Board::from_fen(fields[2..8].join(" ").as_str());
            idx = 8;
        }
        _ => return Err(format!("Unrecognized position command: {}", fields[1])),
    }

    if fields.len() - idx >= 2 && fields[idx] == "moves" {
        idx += 1;
        for token in &fields[idx..] {
            if let Ok(m) = board.find_move(token) {
                board.make_move(m);
            } else {
                return Err(format!("Invalid move: {token}"));
            }
        }
    }

    Ok(board)
}

fn go(board: &Board, fields: &[&str]) -> Result<(Arc<AtomicBool>, JoinHandle<()>), String> {
    let mut limits = SearchLimits::new();

    let mut idx = 1;
    while idx < fields.len() {
        let token = fields[idx];

        #[allow(clippy::match_same_arms)]
        match token {
            "searchmoves" => {}
            "ponder" => {}
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
            "movestogo" => {}
            "depth" => {
                idx += 1;
                limits = limits.depth(parse_value(fields[idx], token));
            }
            "nodes" => {
                idx += 1;
                limits = limits.nodes(parse_value(fields[idx], token));
            }
            "mate" => {}
            "movetime" => {
                idx += 1;
                limits = limits.movetime(parse_value(fields[idx], token));
            }
            "infinite" => {
                limits = limits.depth(None);
            }
            _ => return Err("Invalid go command!".to_string()),
        }

        idx += 1;
    }

    let mut search = Search::new(board, &SimpleEvaluator::new(), Some(limits));
    let is_running = search.get_running();
    let join_handle = thread::spawn(move || {
        search.search(None);
    });

    Ok((is_running, join_handle))
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
