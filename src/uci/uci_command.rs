use crate::search::limits::SearchLimits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UCICommand {
    Uci,
    IsReady,
    UCINewGame,
    SetOption {
        name: String,
        value: Option<String>,
    },
    Position {
        kind: PositionKind,
        moves: Option<Vec<String>>,
    },
    Go {
        limits: SearchLimits,
    },
    Stop,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionKind {
    StartPos,
    Fen { fen: String },
}

impl UCICommand {
    pub fn new(args: &[&str]) -> Result<Self, String> {
        if args.is_empty() {
            return Err("No command specified!".to_string());
        }

        let command = args[0];
        match command {
            "uci" => Ok(Self::Uci),
            "isready" => Ok(Self::IsReady),
            "ucinewgame" => Ok(Self::UCINewGame),
            "setoption" => Self::parse_option(&args[1..]),
            "position" => Self::parse_position(&args[1..]),
            "go" => {
                Self::parse_go(&args[1..]).map_err(|e| format!("Failed to parse go command: {e}"))
            }
            "stop" => Ok(Self::Stop),
            "quit" => Ok(Self::Quit),
            _ => Err(format!("Unrecognized command: {command}")),
        }
    }

    fn parse_option(args: &[&str]) -> Result<Self, String> {
        if args.len() < 2 {
            return Err("Not enough arguments for setoption".to_string());
        }

        let name_idx = args
            .iter()
            .position(|&arg| arg == "name")
            .ok_or("No name provided to setoption!")?;
        if args.len() < name_idx + 2 {
            return Err("Not enough arguments for setoption".to_string());
        }

        let value_idx = args.iter().position(|&arg| arg == "value");
        let value = match value_idx {
            Some(idx) if args.len() > idx => Some(args[idx + 1..].join(" ").to_lowercase()),
            Some(_) => {
                return Err(
                    "No value provided but value command specified to setoption!".to_string(),
                )
            }
            None => None,
        };

        let name = value_idx
            .map_or_else(
                || args[name_idx + 1..].join(" "),
                |end_idx| args[name_idx + 1..end_idx].join(" "),
            )
            .to_lowercase();

        assert!(!name.is_empty(), "Name should not be empty!");

        Ok(Self::SetOption { name, value })
    }

    fn parse_position(args: &[&str]) -> Result<Self, String> {
        if args.is_empty() {
            return Err("No position specified!".to_string());
        }

        let kind = match args[0] {
            "startpos" => PositionKind::StartPos,
            "fen" => {
                if args.len() < 7 {
                    return Err("No FEN specified!".to_string());
                }
                PositionKind::Fen {
                    fen: args[1..7].join(" "),
                }
            }
            _ => return Err(format!("Unrecognized position command: {}", args[0])),
        };

        let moves = match kind {
            PositionKind::StartPos if args.len() > 2 && args[1] == "moves" => {
                Some(args[2..].iter().map(ToString::to_string).collect())
            }
            PositionKind::Fen { .. } if args.len() > 8 && args[7] == "moves" => {
                Some(args[8..].iter().map(ToString::to_string).collect())
            }
            _ => None,
        };

        Ok(Self::Position { kind, moves })
    }

    fn parse_go(args: &[&str]) -> Result<Self, String> {
        let mut limits = SearchLimits::new();

        let mut idx = 0;
        while idx < args.len() {
            let token = args[idx];

            #[allow(clippy::match_same_arms)]
            match token {
                "perft" => {
                    limits.perft = true;
                }
                "searchmoves" => {}
                "ponder" => {}
                "wtime" => {
                    idx += 1;
                    limits = limits.white_time(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse wtime value: {e}"))?,
                    ));
                }
                "btime" => {
                    idx += 1;
                    limits = limits.black_time(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse btime value: {e}"))?,
                    ));
                }
                "winc" => {
                    idx += 1;
                    limits = limits.white_increment(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse winc value: {e}"))?,
                    ));
                }
                "binc" => {
                    idx += 1;
                    limits = limits.black_increment(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse binc value: {e}"))?,
                    ));
                }
                "movestogo" => {}
                "depth" => {
                    idx += 1;
                    limits = limits.depth(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse depth value: {e}"))?,
                    ));
                }
                "nodes" => {
                    idx += 1;
                    limits = limits.nodes(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse nodes value: {e}"))?,
                    ));
                }
                "mate" => {}
                "movetime" => {
                    idx += 1;
                    limits = limits.movetime(Some(
                        args[idx]
                            .parse()
                            .map_err(|e| format!("Failed to parse movetime value: {e}"))?,
                    ));
                }
                "infinite" => {
                    return Ok(Self::Go {
                        limits: SearchLimits::new(),
                    });
                }
                _ => return Err("Invalid go command!".to_string()),
            }

            idx += 1;
        }

        Ok(Self::Go { limits })
    }
}
