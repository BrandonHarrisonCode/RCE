use super::ply::Ply;
use super::Board;

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
    #[allow(dead_code)]
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
    pub fn history(mut self, history: &[Ply]) -> BoardBuilder {
        self.history = history.to_vec();
        self
    }

    #[allow(dead_code)]
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