use build_time::build_time_utc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::bench;
use crate::board::{Board, BoardBuilder};
use crate::evaluate::simple_evaluator::SimpleEvaluator;
use crate::search::{limits::SearchLimits, Depth, Search};

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
            "bench" => bench::bench(),
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
            "setoption" => {} // Currently changing options is not supported
            "debug" => println!("Not supported"),
            _ => println!("Invalid command!"),
        }
    }
}

/// Prints information about the engine like the name and options supported.
fn print_engine_info() {
    println!("id name {TITLE} {VERSION}");
    println!("id author {AUTHOR}");
    // Print options here
    println!("option name Threads type spin default 1 min 1 max 1");
    println!("option name Hash type spin default 1 min 1 max 1");
    println!("option name Move Overhead type spin default 10 min 0 max 5000");
    println!("uciok");
}

/// Loads the position from the UCI command. It can be either a FEN string or a starting position.
/// It can also take an optional list of move to make from the original position.
/// It expects these moves to be in longform algebraic notation, e.g. b2b4, e7a7, etc.
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

/// Runs a search using the specified limits.
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
                limits = limits.white_time(parse_value(fields[idx]));
            }
            "btime" => {
                idx += 1;
                limits = limits.black_time(parse_value(fields[idx]));
            }
            "winc" => {
                idx += 1;
                limits = limits.white_increment(parse_value(fields[idx]));
            }
            "binc" => {
                idx += 1;
                limits = limits.black_increment(parse_value(fields[idx]));
            }
            "movestogo" => {}
            "depth" => {
                idx += 1;
                limits = limits.depth(parse_value(fields[idx]));
            }
            "nodes" => {
                idx += 1;
                limits = limits.nodes(parse_value(fields[idx]));
            }
            "mate" => {}
            "movetime" => {
                idx += 1;
                limits = limits.movetime(parse_value(fields[idx]));
            }
            "infinite" => {
                limits = limits.depth(None);
            }
            _ => return Err("Invalid go command!".to_string()),
        }

        idx += 1;
    }

    let max_depth: Option<Depth> = limits
        .depth
        .map(|d| Depth::try_from(d).unwrap_or(Depth::MAX));
    let mut search = Search::new(board, &SimpleEvaluator::new(), Some(limits));
    let is_running = search.get_running();
    let join_handle = thread::spawn(move || {
        search.search(max_depth);
    });

    Ok((is_running, join_handle))
}

/// Parses a value from a string and returns it as the specified type.
/// If the parsing fails, it prints an error message and returns None.
///
/// # Arguments
/// * `str` - The string to parse.
///
/// # Returns
/// * `Some(T)` if parsing was successful.
/// * `None` if parsing failed.
///
/// # Example
/// ```
/// let value: Option<u32> = parse_value("42", "u32");
/// assert_eq!(value, Some(42));
/// ```
fn parse_value<T>(str: &str) -> Option<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let result = str
        .parse()
        .map_err(|e| format!("Failed to parse value \"{e}\" for token!"));
    if let Err(err_str) = result {
        eprintln!("{err_str}");
        return None;
    }

    Some(result.unwrap())
}
