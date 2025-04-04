mod uci_command;

use build_time::build_time_utc;
use std::io::BufRead;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};

use crate::board::{Board, BoardBuilder};
use crate::evaluate::simple_evaluator::SimpleEvaluator;
use crate::logger::Logger;
use crate::search::{limits::SearchLimits, Depth, Search};
use uci_command::{PositionKind, UCICommand};

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

const VERSION: &str = build_time_utc!("%Y.%m.%d %H:%M:%S");

pub fn start() {
    UCI::new()
        .uci_loop(&mut std::io::stdin().lock())
        .unwrap_or_else(|err| {
            eprintln!("{err}");
        });
}

struct UCI {
    board: Board,
    search_running: Option<Arc<AtomicBool>>,
    join_handle: Option<JoinHandle<()>>,
}

impl Logger for UCI {}

impl UCI {
    fn new() -> Self {
        Self {
            board: BoardBuilder::construct_starting_board().build(),
            search_running: None,
            join_handle: None,
        }
    }

    fn uci_loop(&mut self, input: &mut impl BufRead) -> Result<(), String> {
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

        Ok(())
    }

    fn execute_command(&mut self, command: UCICommand) -> Result<(), String> {
        #[allow(clippy::match_same_arms)]
        match command {
            UCICommand::UCI => self.print_engine_info(),
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
                if let Ok((new_search, new_join_handle)) = self.go(limits) {
                    self.search_running = Some(new_search);
                    self.join_handle = Some(new_join_handle);
                } else {
                    return Err("Failed to start search".to_string());
                }
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
                self.setoption(name, value).unwrap_or_else(|err| {
                    self.elog(format!("Failed to set option: {err}"));
                })
            } // Currently changing options is not supported
        }

        Ok(())
    }

    /// Prints information about the engine like the name and options supported.
    fn print_engine_info(&self) {
        self.log(format!("id name {TITLE} {VERSION}"));
        self.log(format!("id author {AUTHOR}"));
        // Print options here
        self.log("option name Threads type spin default 1 min 1 max 1");
        self.log("option name Hash type spin default 1 min 1 max 1");
        self.log("option name Move Overhead type spin default 10 min 0 max 5000");
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
    fn go(&mut self, limits: SearchLimits) -> Result<(Arc<AtomicBool>, JoinHandle<()>), String> {
        let max_depth: Option<Depth> = limits
            .depth
            .map(|d| Depth::try_from(d).unwrap_or(Depth::MAX));
        let mut search = Search::new(&self.board, Some(limits));
        let is_running = search.running.clone();
        let join_handle = thread::spawn(move || {
            search.search(&SimpleEvaluator, max_depth);
        });

        Ok((is_running, join_handle))
    }

    fn setoption(&mut self, _name: String, _value: Option<String>) -> Result<(), String> {
        // Currently not implemented
        self.log(format!("{_name}"));
        self.log(format!("{_value:?}"));
        Err("Setting options is not implemented yet".to_string())
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
        let mut uci = UCI::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::UCI);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_isready() {
        let input = "isready\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = UCI::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::IsReady);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ucinewgame() {
        let input = "ucinewgame\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = UCI::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::UCINewGame);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stop() {
        let input = "stop\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(command, UCICommand::Quit);

        let result = uci.execute_command(command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_position_startpos() {
        let input = "position startpos\n".to_string();
        let fields: Vec<_> = input.trim().split_whitespace().collect();
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

        let command = UCICommand::new(&fields).unwrap();
        assert_eq!(
            command,
            UCICommand::Go {
                limits: SearchLimits {
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
        let mut uci = UCI::new();

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
        let mut uci = UCI::new();

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
