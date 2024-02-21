use crate::utils::FENInstruction;

use super::{Board, Castling, Color, Ply, Square};

impl Board {
    /// Returns a new board given a FEN string
    ///
    /// # Examples
    /// ```
    /// let board = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");
    /// ```
    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Self {
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
                _ => panic!("Unknown FEN instruction: {chr}"),
            };

            let mask: u64 = 1 << (63 - idx);
            match instruction {
                FENInstruction::Bitboard(bb) => *bb |= mask,
                FENInstruction::Skip(num) => idx += num - 1,
                FENInstruction::NewRow() => idx -= 1,
            }
            idx += 1;
        }

        let current_turn = match fields[1].chars().next().unwrap_or('w') {
            'w' => Color::White,
            'b' => Color::Black,
            _ => panic!("Not given a valid FEN. The second field must either be a 'b' or a 'w'"),
        };

        let mut w_kingside_castling: Castling = Castling::Unavailiable;
        let mut b_kingside_castling: Castling = Castling::Unavailiable;
        let mut w_queenside_castling: Castling = Castling::Unavailiable;
        let mut b_queenside_castling: Castling = Castling::Unavailiable;

        for chr in fields[2].chars() {
            match chr {
                'K' => w_kingside_castling = Castling::Availiable,
                'k' => b_kingside_castling = Castling::Availiable,
                'Q' => w_queenside_castling = Castling::Availiable,
                'q' => b_queenside_castling = Castling::Availiable,
                '-' => (),
                _ => panic!("Unknown FEN castling notation: {chr}"),
            };
        }

        #[allow(clippy::cast_possible_truncation)]
        let en_passant_file = match fields[3].chars().next().unwrap_or('-') {
            '-' => None,
            'a'..='h' => Some((fields[3].chars().next().unwrap() as u128 - 'a' as u128) as u8),
            _ => panic!("Unknown FEN en passant notation: {}", fields[3]),
        };

        // If the first move is en passant, add the previous pawn push to the
        // history so we can restore en passant rights
        let mut history = Vec::new();
        if let Some(file) = en_passant_file {
            let (start, dest) = match current_turn {
                Color::White => (Square { rank: 1, file }, Square { rank: 3, file }),
                Color::Black => (Square { rank: 6, file }, Square { rank: 4, file }),
            };

            history.push(Ply::builder(start, dest).en_passant(true).build());
        }

        // TODO: Halfmove clock
        // TODO: Fullmove number

        Self {
            current_turn,

            w_kingside_castling,
            w_queenside_castling,
            b_kingside_castling,
            b_queenside_castling,

            en_passant_file,

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

            history,
        }
    }
}
