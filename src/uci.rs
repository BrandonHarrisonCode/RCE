use crate::board::{Board, BoardBuilder};

const TITLE: &str = "Rust Chess Engine";
const AUTHOR: &str = "Brandon Harrison";

pub fn start() {
    let mut board = BoardBuilder::construct_starting_board();

    loop {
        println!("{board}");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        let fields: Vec<&str> = trimmed.split_whitespace().collect();
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
