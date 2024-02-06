use super::piece::Color;
use super::ply::Ply;
use super::Board;
use super::Castling;

#[derive(Default)]
pub struct BoardBuilder {
    is_white_turn: bool,

    w_kingside_castling: Castling,
    w_queenside_castling: Castling,
    b_kingside_castling: Castling,
    b_queenside_castling: Castling,

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
    pub const fn default() -> Self {
        Self {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
    pub const fn white_turn(mut self, white: bool) -> Self {
        self.is_white_turn = white;
        self
    }

    #[allow(dead_code)]
    pub const fn kingside_castling(mut self, color: Color, value: Castling) -> Self {
        match color {
            Color::White => self.w_kingside_castling = value,
            Color::Black => self.b_kingside_castling = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn queenside_castling(mut self, color: Color, value: Castling) -> Self {
        match color {
            Color::White => self.w_queenside_castling = value,
            Color::Black => self.b_queenside_castling = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn pawns(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_pawns = value,
            Color::Black => self.b_pawns = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn king(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_king = value,
            Color::Black => self.b_king = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn queens(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_queens = value,
            Color::Black => self.b_queens = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn rooks(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_rooks = value,
            Color::Black => self.b_rooks = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn bishops(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_bishops = value,
            Color::Black => self.b_bishops = value,
        }
        self
    }

    #[allow(dead_code)]
    pub const fn knights(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_knights = value,
            Color::Black => self.b_knights = value,
        }
        self
    }

    #[allow(dead_code)]
    pub fn history(mut self, history: &[Ply]) -> Self {
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
    use super::super::square::Square;
    use super::*;

    #[test]
    fn board_builder_default() {
        let board = BoardBuilder::default().build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .white_turn(false)
            .white_turn(true)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .kingside_castling(Color::White, Castling::Unavailiable)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Unavailiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .kingside_castling(Color::Black, Castling::Unavailiable)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Unavailiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .queenside_castling(Color::White, Castling::Unavailiable)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Unavailiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .queenside_castling(Color::Black, Castling::Unavailiable)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Unavailiable,

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
        let board = BoardBuilder::default()
            .pawns(Color::White, 1)
            .pawns(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .king(Color::White, 1)
            .king(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .queens(Color::White, 1)
            .queens(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .rooks(Color::White, 1)
            .rooks(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .bishops(Color::White, 1)
            .bishops(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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
        let board = BoardBuilder::default()
            .knights(Color::White, 1)
            .knights(Color::Black, 2)
            .build();
        let correct = Board {
            is_white_turn: true,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

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

            history,
        };

        assert_eq!(board, correct);
    }
}
