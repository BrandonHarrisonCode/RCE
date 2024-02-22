use super::piece::Color;
use super::ply::Ply;
use super::Board;
use super::Castling;

#[derive(Default)]
pub struct BoardBuilder {
    pub current_turn: Color,
    pub halfmove_clock: u8,
    pub fullmove_counter: u16,

    pub w_kingside_castling: Castling,
    pub w_queenside_castling: Castling,
    pub b_kingside_castling: Castling,
    pub b_queenside_castling: Castling,

    pub en_passant_file: Option<u8>,

    pub w_pawns: u64,
    pub w_king: u64,
    pub w_queens: u64,
    pub w_rooks: u64,
    pub w_bishops: u64,
    pub w_knights: u64,
    pub b_pawns: u64,
    pub b_king: u64,
    pub b_queens: u64,
    pub b_rooks: u64,
    pub b_bishops: u64,
    pub b_knights: u64,

    pub history: Vec<Ply>,
}

impl BoardBuilder {
    #[allow(dead_code)]
    pub const fn default() -> Self {
        Self {
            current_turn: Color::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,

            w_kingside_castling: Castling::Availiable,
            w_queenside_castling: Castling::Availiable,
            b_kingside_castling: Castling::Availiable,
            b_queenside_castling: Castling::Availiable,

            en_passant_file: None,

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

    /// Set the color of the player who is currently playing
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the player who is currently playing
    ///
    /// # Returns
    ///
    /// * `Self` - The current builder
    ///
    /// # Example
    ///
    /// ```
    /// use chess::board::BoardBuilder;
    /// use chess::piece::Color;
    ///
    /// let builder = BoardBuilder::default().white_turn(false);
    /// ```
    #[allow(dead_code)]
    pub const fn turn(mut self, color: Color) -> Self {
        self.current_turn = color;
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

    pub const fn pawns(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_pawns = value,
            Color::Black => self.b_pawns = value,
        }
        self
    }

    pub const fn king(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_king = value,
            Color::Black => self.b_king = value,
        }
        self
    }

    pub const fn queens(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_queens = value,
            Color::Black => self.b_queens = value,
        }
        self
    }

    pub const fn rooks(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_rooks = value,
            Color::Black => self.b_rooks = value,
        }
        self
    }

    pub const fn bishops(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_bishops = value,
            Color::Black => self.b_bishops = value,
        }
        self
    }

    pub const fn knights(mut self, color: Color, value: u64) -> Self {
        match color {
            Color::White => self.w_knights = value,
            Color::Black => self.b_knights = value,
        }
        self
    }

    pub fn history(mut self, history: &[Ply]) -> Self {
        self.history = history.to_vec();
        self
    }

    pub const fn en_passant_file(mut self, en_passant_file: Option<u8>) -> Self {
        self.en_passant_file = en_passant_file;
        self
    }

    pub const fn halfmove_clock(mut self, value: u8) -> Self {
        self.halfmove_clock = value;
        self
    }

    pub const fn fullmove_counter(mut self, value: u16) -> Self {
        self.fullmove_counter = value;
        self
    }

    #[allow(dead_code)]
    pub fn build(&mut self) -> Board {
        Board {
            current_turn: self.current_turn,
            halfmove_clock: self.halfmove_clock,
            fullmove_counter: self.fullmove_counter,

            w_kingside_castling: self.w_kingside_castling,
            w_queenside_castling: self.w_queenside_castling,
            b_kingside_castling: self.b_kingside_castling,
            b_queenside_castling: self.b_queenside_castling,

            en_passant_file: self.en_passant_file,

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
        let correct = Board::construct_empty_board();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_turn() {
        let board = BoardBuilder::default().turn(Color::Black).build();
        let correct = Board {
            current_turn: Color::Black,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_turn() {
        let board = BoardBuilder::default()
            .turn(Color::Black)
            .turn(Color::White)
            .build();
        let correct = Board::construct_empty_board();

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_kingside_castling() {
        let board = BoardBuilder::default()
            .kingside_castling(Color::White, Castling::Unavailiable)
            .build();
        let correct = Board {
            w_kingside_castling: Castling::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_kingside_castling() {
        let board = BoardBuilder::default()
            .kingside_castling(Color::Black, Castling::Unavailiable)
            .build();
        let correct = Board {
            b_kingside_castling: Castling::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_white_queenside_castling() {
        let board = BoardBuilder::default()
            .queenside_castling(Color::White, Castling::Unavailiable)
            .build();
        let correct = Board {
            w_queenside_castling: Castling::Unavailiable,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_black_queenside_castling() {
        let board = BoardBuilder::default()
            .queenside_castling(Color::Black, Castling::Unavailiable)
            .build();
        let correct = Board {
            b_queenside_castling: Castling::Unavailiable,
            ..Board::construct_empty_board()
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
            w_pawns: 1,
            b_pawns: 2,
            ..Board::construct_empty_board()
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
            w_king: 1,
            b_king: 2,
            ..Board::construct_empty_board()
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
            w_queens: 1,
            b_queens: 2,
            ..Board::construct_empty_board()
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
            w_rooks: 1,
            b_rooks: 2,
            ..Board::construct_empty_board()
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
            w_bishops: 1,
            b_bishops: 2,
            ..Board::construct_empty_board()
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
            w_knights: 1,
            b_knights: 2,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_history() {
        let history = vec![Ply::new(Square::new("a1"), Square::new("a2"))];
        let board = BoardBuilder::default().history(&history).build();
        let correct = Board {
            history,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_en_passant() {
        let board = BoardBuilder::default().en_passant_file(Some(1)).build();
        let correct = Board {
            en_passant_file: Some(1),
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_halfmove_clock() {
        let board = BoardBuilder::default().halfmove_clock(5).build();
        let correct = Board {
            halfmove_clock: 5,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }

    #[test]
    fn board_builder_fullmove_counter() {
        let board = BoardBuilder::default().fullmove_counter(5).build();
        let correct = Board {
            fullmove_counter: 5,
            ..Board::construct_empty_board()
        };

        assert_eq!(board, correct);
    }
}
