use super::ply::Ply;
use super::Board;
use crate::utils::FENInstruction;

#[derive(Default)]
pub struct BoardBuilder {
    is_white_turn: bool,

    w_kingside_castling: bool,
    w_queenside_castling: bool,
    b_kingside_castling: bool,
    b_queenside_castling: bool,

    w_pawns: u64,
    w_king: u64,
    w_queens: u64,
    w_rooks: u64,
    w_bishops: u64,
    w_knights: u64,
    b_pawns: u64,
    b_king: u64,
    b_queens: u64,
    b_rooks: u64,
    b_bishops: u64,
    b_knights: u64,

    history: Vec<Ply>,
}

impl BoardBuilder {
    pub fn default() -> BoardBuilder {
        BoardBuilder {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,

            history: Vec::new(),
        }
    }


    #[allow(dead_code)]
    pub fn white_turn(mut self, white: bool) -> BoardBuilder {
        self.is_white_turn = white;
        self
    }

    #[allow(dead_code)]
    pub fn kingside_castling(mut self, white: bool, value: bool) -> BoardBuilder {
        match white {
            true => self.w_kingside_castling = value,
            false => self.b_kingside_castling = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn queenside_castling(mut self, white: bool, value: bool) -> BoardBuilder {
        match white {
            true => self.w_queenside_castling = value,
            false => self.b_queenside_castling = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn pawns(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_pawns = value,
            false => self.b_pawns = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn king(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_king = value,
            false => self.b_king = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn queens(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_queens = value,
            false => self.b_queens = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn rooks(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_rooks = value,
            false => self.b_rooks = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn bishops(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_bishops = value,
            false => self.b_bishops = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn knights(mut self, white: bool, value: u64) -> BoardBuilder {
        match white {
            true => self.w_knights = value,
            false => self.b_knights = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn history(mut self, history: &Vec<Ply>) -> BoardBuilder {
        self.history = history.clone();
        self
    }

    /// Rewhite_turns a new board given a FEN string
    ///
    /// # Arguments
    ///
    /// * `fen` - A string representing the FEN of a position
    ///
    /// # Examples
    /// ```
    /// // Create empty board
    /// let board = Board::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    /// ```
    #[allow(dead_code)]
    pub fn from_fen(&mut self, fen: &str) -> Board {
        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();

        let mut w_pawns: u64 = 0;
        let mut w_king: u64 = 0;
        let mut w_queens: u64 = 0;
        let mut w_rooks: u64 = 0;
        let mut w_bishops: u64 = 0;
        let mut w_knights: u64 = 0;
        let mut b_pawns: u64 = 0;
        let mut b_king: u64 = 0;
        let mut b_queens: u64 = 0;
        let mut b_rooks: u64 = 0;
        let mut b_bishops: u64 = 0;
        let mut b_knights: u64 = 0;

        let mut idx: u64 = 0;
        for chr in fields[0].chars() {
            let instruction = match chr {
                'P' => FENInstruction::Bitboard(&mut w_pawns),
                'K' => FENInstruction::Bitboard(&mut w_king),
                'Q' => FENInstruction::Bitboard(&mut w_queens),
                'R' => FENInstruction::Bitboard(&mut w_rooks),
                'B' => FENInstruction::Bitboard(&mut w_bishops),
                'N' => FENInstruction::Bitboard(&mut w_knights),
                'p' => FENInstruction::Bitboard(&mut b_pawns),
                'k' => FENInstruction::Bitboard(&mut b_king),
                'q' => FENInstruction::Bitboard(&mut b_queens),
                'r' => FENInstruction::Bitboard(&mut b_rooks),
                'b' => FENInstruction::Bitboard(&mut b_bishops),
                'n' => FENInstruction::Bitboard(&mut b_knights),
                '1'..='8' => FENInstruction::Skip(chr.to_string().parse().ok().unwrap()),
                '/' => FENInstruction::NewRow(),
                _ => panic!("Unknown FEN instruction: {}", chr),
            };

            let mask: u64 = 1 << (63 - idx);
            match instruction {
                FENInstruction::Bitboard(bb) => *bb |= mask,
                FENInstruction::Skip(num) => idx += num - 1,
                FENInstruction::NewRow() => idx -= 1,
            }
            idx += 1;
        }

        let is_white_turn = match fields[1].chars().next().unwrap_or('w') {
            'w' => true,
            'b' => false,
            _ => panic!("Not given a valid FEN. The second field must either be a 'b' or a 'w'"),
        };

        let mut w_kingside_castling: bool = false;
        let mut b_kingside_castling: bool = false;
        let mut w_queenside_castling: bool = false;
        let mut b_queenside_castling: bool = false;

        for chr in fields[2].chars() {
            match chr {
                'K' => w_kingside_castling = true,
                'k' => b_kingside_castling = true,
                'Q' => w_queenside_castling = true,
                'q' => b_queenside_castling = true,
                '-' => (),
                _ => panic!("Unknown FEN castling notation: {}", chr),
            };
        };

        // TODO: Castling rights
        // TODO: En passant target square
        // TODO: Halfmove clock
        // TODO: Fullmove number

        Board {
            is_white_turn,

            w_kingside_castling,
            w_queenside_castling,
            b_kingside_castling,
            b_queenside_castling,

            history: Vec::new(),
            w_pawns,
            w_king,
            w_queens,
            w_rooks,
            w_bishops,
            w_knights,
            b_pawns,
            b_king,
            b_queens,
            b_rooks,
            b_bishops,
            b_knights,
        }
    }

    /// Creates a new board object that represents the starting board state in a normal game
    pub fn starting_board(&mut self) -> Board {
        Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            w_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
            w_queens: 0b0000000000000000000000000000000000000000000000000000000000010000,
            w_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            w_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            w_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            b_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            b_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
            b_queens: 0b0001000000000000000000000000000000000000000000000000000000000000,
            b_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            b_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            b_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,

            history: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Board {
        Board {
            is_white_turn: self.is_white_turn,

            w_kingside_castling: self.w_kingside_castling,
            w_queenside_castling: self.w_queenside_castling,
            b_kingside_castling: self.b_kingside_castling,
            b_queenside_castling: self.b_queenside_castling,

            history: self.history.clone(),
            w_pawns: self.w_pawns,
            w_king: self.w_king,
            w_queens: self.w_queens,
            w_rooks: self.w_rooks,
            w_bishops: self.w_bishops,
            w_knights: self.w_knights,
            b_pawns: self.b_pawns,
            b_king: self.b_king,
            b_queens: self.b_queens,
            b_rooks: self.b_rooks,
            b_bishops: self.b_bishops,
            b_knights: self.b_knights,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::square::Square;

    #[test]
    fn from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Board::new().starting_board(), Board::new().from_fen(fen));
    }

    #[test]
    fn from_fen_white_position1() {
        let fen = "1k1r3r/p6p/1pp1pp2/2Np1qp1/1Q1P4/2P1PP2/PP4PP/R4nK1 w - - 0 21";
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: false,
            w_queenside_castling: false,
            b_kingside_castling: false,
            b_queenside_castling: false,

            w_pawns: 271368960,
            w_king: 2,
            w_queens: 1073741824,
            w_rooks: 128,
            w_bishops: 0,
            w_knights: 137438953472,
            b_pawns: 36429096560885760,
            b_king: 4611686018427387904,
            b_queens: 17179869184,
            b_rooks: 1224979098644774912,
            b_bishops: 0,
            b_knights: 4,
            history: Vec::new(),
        };

        assert_eq!(Board::new().from_fen(fen), correct);
    }

    #[test]
    fn from_fen_black_position1() {
        let fen = "5b2/pp1N2pk/2pn1q1p/3n1p1Q/3P1P2/2PB3R/PP3KPP/R1B1r3 b - - 12 31";
        let correct = Board {
            is_white_turn: false,

            w_kingside_castling: false,
            w_queenside_castling: false,
            b_kingside_castling: false,
            b_queenside_castling: false,

            w_pawns: 337691392,
            w_king: 1024,
            w_queens: 4294967296,
            w_rooks: 65664,
            w_bishops: 1048608,
            w_knights: 4503599627370496,
            b_pawns: 54642446545453056,
            b_king: 281474976710656,
            b_queens: 4398046511104,
            b_rooks: 8,
            b_bishops: 288230376151711744,
            b_knights: 17660905521152,
            history: Vec::new(),
        };

        assert_eq!(Board::new().from_fen(fen), correct);
    }

    #[test]
    fn board_builder_default() {
        let board = BoardBuilder::default().build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_white_turn() {
        let board = BoardBuilder::default().white_turn(false).build();
        let correct = Board {
            is_white_turn: false,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_turn() {
        let board = BoardBuilder::default().white_turn(false).white_turn(true).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_kingside_castling() {
        let board = BoardBuilder::default().kingside_castling(true, false).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: false,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_kingside_castling() {
        let board = BoardBuilder::default().kingside_castling(false, false).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: false,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_queenside_castling() {
        let board = BoardBuilder::default().queenside_castling(true, false).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: false,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_queenside_castling() {
        let board = BoardBuilder::default().queenside_castling(false, false).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: false,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_pawns() {
        let board = BoardBuilder::default().pawns(true, 1).pawns(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 1,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 2,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_king() {
        let board = BoardBuilder::default().king(true, 1).king(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 1,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 2,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_queens() {
        let board = BoardBuilder::default().queens(true, 1).queens(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 1,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 2,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_rooks() {
        let board = BoardBuilder::default().rooks(true, 1).rooks(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 1,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 2,
            b_bishops: 0,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_bishops() {
        let board = BoardBuilder::default().bishops(true, 1).bishops(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 1,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 2,
            b_knights: 0,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_knights() {
        let board = BoardBuilder::default().knights(true, 1).knights(false, 2).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 1,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 2,
            history: Vec::new(),
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_history() {
        let history = vec![Ply::new(Square::new("a1"), Square::new("a2"))];
        let board = BoardBuilder::default().history(&history).build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: true,
            w_queenside_castling: true,
            b_kingside_castling: true,
            b_queenside_castling: true,

            w_pawns: 0,
            w_king: 0,
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            b_pawns: 0,
            b_king: 0,
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            history: history,
        };

        assert_eq!(board, correct);
    }
}