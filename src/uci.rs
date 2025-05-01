mod uci_command;
mod uci_option;

use build_time::build_time_utc;
use parking_lot::RwLock;
use std::io::BufRead;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};
use uci_option::{OptionType, UCIOption};

use crate::board::transposition_table::TranspositionTable;
use crate::board::{Board, BoardBuilder};
use crate::evaluate::simple_evaluator::SimpleEvaluator;
use crate::logger::Logger;
use crate::search::{limits::SearchLimits, Depth, Search};
use crate::testing_utils::perft;
use uci_command::{PositionKind, UCICommand};

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

const VERSION: &str = build_time_utc!("%Y.%m.%d %H:%M:%S");

pub fn start() {
    Uci::new().uci_loop(&mut std::io::stdin().lock());
}

struct Uci {
    board: Board,
    search_running: Option<Arc<AtomicBool>>,
    join_handle: Option<JoinHandle<()>>,
    config: Config,
    transposition_table: Arc<RwLock<TranspositionTable>>,
}

struct Config {
    pub hash_size: u64,
    pub move_overhead: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hash_size: crate::board::transposition_table::DEFAULT_SIZE_IN_MB,
            move_overhead: 10,
        }
    }
}

impl Logger for Uci {}

impl Uci {
    fn new() -> Self {
        Self {
            board: BoardBuilder::construct_starting_board().build(),
            search_running: None,
            join_handle: None,
            config: Config::default(),
            transposition_table: Arc::new(RwLock::new(TranspositionTable::default())),
        }
    }

    fn uci_loop(&mut self, input: &mut impl BufRead) {
        loop {
            let mut line = String::new();
            input.read_line(&mut line).unwrap();
            let trimmed = line.trim();
            let fields: Vec<_> = trimmed.split_whitespace().collect();

            let command = match UCICommand::new(&fields) {
                Ok(cmd) => cmd,
                Err(err) => {
                    self.elog(format!("Failed to parse command: {err}"));
                    continue;
                }
            };

            if matches!(command, UCICommand::Quit) {
                break;
            }

            self.execute_command(command).unwrap_or_else(|err| {
                self.elog(format!("Failed to execute command: {err}"));
            });
        }
    }

    fn execute_command(&mut self, command: UCICommand) -> Result<(), String> {
        #[allow(clippy::match_same_arms)]
        match command {
            UCICommand::Uci => self.print_engine_info(),
            UCICommand::IsReady => self.log("readyok"),
            UCICommand::UCINewGame => self.board = BoardBuilder::construct_starting_board().build(),
            UCICommand::Position { kind, moves } => self
                .load_position(kind, moves)
                .map_err(|err| format!("Failed to load position: {err}"))?,
            UCICommand::Go { limits } => {
                if let Some(jh) = &self.join_handle {
                    if !jh.is_finished() {
                        return Err("Search is already running".to_string());
                    }
                }
                self.go(limits, &self.transposition_table.clone());
            }
            UCICommand::Stop => {
                if let Some(is_running) = &self.search_running {
                    is_running.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            }
            UCICommand::Quit => {
                unreachable!("Quit command should be handled in the main loop")
            }
            UCICommand::SetOption { name, value } => {
                self.setoption(&name, value.as_ref()).unwrap_or_else(|err| {
                    self.elog(format!("Failed to set option: {err}"));
                });
            } // Currently changing options is not supported
        }

        Ok(())
    }

    /// Prints information about the engine like the name and options supported.
    fn print_engine_info(&self) {
        let supported_options = [
            UCIOption::new(
                "Threads",
                OptionType::Spin,
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
            ),
            UCIOption::new(
                "Hash",
                OptionType::Spin,
                Some("1".to_string()),
                Some("32000".to_string()),
                Some(format!("{}", self.config.hash_size)),
            ),
            UCIOption::new(
                "Move Overhead",
                OptionType::Spin,
                Some("0".to_string()),
                Some("5000".to_string()),
                Some(format!("{}", self.config.move_overhead)),
            ),
            UCIOption::new("Clear Hash", OptionType::Button, None, None, None),
        ];

        self.log(format!("id name {TITLE} {VERSION}"));
        self.log(format!("id author {AUTHOR}"));

        // Print options here
        for option in supported_options {
            self.log(format!("{option}"));
        }

        self.log("uciok");
    }

    /// Loads the position from the UCI command. It can be either a FEN string or a starting position.
    /// It can also take an optional list of move to make from the original position.
    /// It expects these moves to be in longform algebraic notation, e.g. b2b4, e7a7, etc.
    fn load_position(
        &mut self,
        kind: PositionKind,
        moves: Option<Vec<String>>,
    ) -> Result<(), String> {
        let mut board = match kind {
            PositionKind::StartPos => BoardBuilder::construct_starting_board().build(),
            PositionKind::Fen { fen } => Board::from_fen(fen.as_str()),
        };

        for notation in moves.unwrap_or_default() {
            if let Ok(m) = board.find_move(notation.as_str()) {
                board.make_move(m);
            } else {
                return Err(format!("Invalid move: {notation}"));
            }
        }

        self.board = board;
        Ok(())
    }

    /// Runs a search using the specified limits.
    fn go(&mut self, limits: SearchLimits, transposition_table: &Arc<RwLock<TranspositionTable>>) {
        let max_depth: Option<Depth> = limits
            .depth
            .map(|d| Depth::try_from(d).unwrap_or(Depth::MAX));

        if limits.perft {
            let mut perft_board = self.board.clone();
            self.join_handle = Some(thread::spawn(move || {
                perft(
                    &mut perft_board,
                    max_depth.expect("Depth should be set for perft").into(),
                );
            }));
            return;
        }

        let mut search = Search::new(&self.board, Some(limits), transposition_table.clone());
        self.search_running = Some(search.running.clone());
        self.join_handle = Some(thread::spawn(move || {
            search.search(&SimpleEvaluator, max_depth);
        }));
    }

    fn setoption(&mut self, name: &String, value: Option<&String>) -> Result<(), String> {
        match name.as_str() {
            "threads" => {
                if let Some(v) = value {
                    if v.parse::<u32>().is_err() {
                        return Err(format!("Invalid value for Threads: {v}"));
                    }
                }
            }
            "hash" => {
                if let Some(v) = value {
                    if let Ok(hash_size) = v.parse::<u64>() {
                        self.config.hash_size = hash_size;
                        self.transposition_table.write().resize(hash_size);
                    } else {
                        return Err(format!("Invalid value for Hash: {v}"));
                    }
                } else {
                    return Err("Hash value is required".to_string());
                }
            }
            "move overhead" => {
                // TODO: Pass this to the search thread
                if let Some(v) = value {
                    if let Ok(move_overhead) = v.parse::<u32>() {
                        self.config.move_overhead = move_overhead;
                    } else {
                        return Err(format!("Invalid value for Move Overhead: {v}"));
                    }
                } else {
                    return Err("Move Overhead value is required".to_string());
                }
            }
            "clear hash" => {
                self.transposition_table.write().clear();
            }
            _ => return Err(format!("Unrecognized option: {name}")),
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    #[test]
    fn test_unrecognized() {
        let input = "unrecognized\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();

        let command = UCICommand::new(&fields);
        assert!(command.is_err());
    }

    #[test]
    fn test_uci() {
        let input = "uci\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::Uci);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_isready() {
        let input = "isready\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::IsReady);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ucinewgame() {
        let input = "ucinewgame\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::UCINewGame);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stop() {
        let input = "stop\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::Stop);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "entered unreachable code")]
    fn test_quit() {
        let input = "quit\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::Quit);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_position_startpos() {
        let input = "position startpos\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Position {
                kind: PositionKind::StartPos,
                moves: None
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_position_startpos_with_moves() {
        let input = "position startpos moves e2e4 e7e5 d2d4 d7d5\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Position {
                kind: PositionKind::StartPos,
                moves: Some(vec![
                    "e2e4".to_string(),
                    "e7e5".to_string(),
                    "d2d4".to_string(),
                    "d7d5".to_string()
                ])
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_position_fen() {
        let input = "position fen 8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Position {
                kind: PositionKind::Fen {
                    fen: String::from("8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42")
                },
                moves: None
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_position_fen_with_moves() {
        let input =
            "position fen 8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42 moves f4f5 h7g8 f5c8 g8h7\n"
                .to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Position {
                kind: PositionKind::Fen {
                    fen: String::from("8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42")
                },
                moves: Some(vec![
                    "f4f5".to_string(),
                    "h7g8".to_string(),
                    "f5c8".to_string(),
                    "g8h7".to_string()
                ])
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_go() {
        let input = "go\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Go {
                limits: SearchLimits::new()
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_go_all() {
        let input = "go wtime 1 btime 2 winc 3 binc 4 depth 5 nodes 6 movetime 7\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Go {
                limits: SearchLimits {
                    perft: false,
                    white_time: Some(1),
                    black_time: Some(2),
                    white_increment: Some(3),
                    black_increment: Some(4),
                    depth: Some(5),
                    nodes: Some(6),
                    movetime: Some(7),

                    time_management_timer: None,
                }
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_go_infinite() {
        let input = "go infinite\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Go {
                limits: SearchLimits::new()
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_go_threefold() {
        let input =
            "position fen 8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42 moves f4f5 h7g8 f5c8 g8h7"
                .to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = Uci::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Position {
                kind: PositionKind::Fen {
                    fen: String::from("8/5ppk/7p/8/P1P1PQ2/8/Pr2N1KR/8 w - - 3 42")
                },
                moves: Some(vec![
                    "f4f5".to_string(),
                    "h7g8".to_string(),
                    "f5c8".to_string(),
                    "g8h7".to_string()
                ])
            }
        );

        let result = uci.execute_command(command);
        assert!(result.is_ok());

        assert!(uci
            .board
            .position_reached(Board::from_fen("8/5ppk/7p/5Q2/P1P1P3/8/Pr2N1KR/8 b - - 4 42").zkey));
    }
}
